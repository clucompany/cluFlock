//! Implementation for platforms with LockFileEx support.

use crate::element::FlockElement;
use crate::err::FlockError;
use crate::err::IoError;
use crate::range::checker::FlockRangePNumBehChecker;
use crate::range::pnum::FlockRangePNumBeh;
use crate::range::FlockRange;
use crate::range::FlockRangeFull;
use crate::unlock::WaitFlockUnlock;
use crate::unlock::WaitFlockUnlockRange;
use crate::ExclusiveFlock;
use crate::FlockLock;
use crate::SharedFlock;
use winapi::shared::minwindef::DWORD;
use winapi::um::minwinbase::LPOVERLAPPED;
use winapi::um::minwinbase::OVERLAPPED;
use winapi::um::winnt::MAXDWORD;
use crate::range::pnum::__make_auto_pnum_type;

//type RawHandle = winapi::um::winnt::HANDLE;
type RawBool = winapi::shared::minwindef::BOOL;
pub type RawFilePtr = winapi::um::winnt::HANDLE;

crate::cfg_std! {
	if #std {
		use std::os::windows::io::AsRawHandle;
		use std::fs::File;

		impl FlockElement for File {
			type FilePtr = RawFilePtr;

			#[inline(always)]
			fn as_file_ptr(&self) -> Self::FilePtr {
				AsRawHandle::as_raw_handle(self) as _
			}
		}
	}else {
		extern crate alloc;
		use alloc::format;
	}
}

mod __internal_flags {
	pub type WinApiFlag = winapi::shared::minwindef::DWORD;

	pub const TRY_EXCLUSIVE_LOCK: WinApiFlag = WAIT_EXCLUSIVE_LOCK | RAW_LOCK_NB;
	pub const WAIT_EXCLUSIVE_LOCK: WinApiFlag = winapi::um::minwinbase::LOCKFILE_EXCLUSIVE_LOCK;

	pub const TRY_SHARED_LOCK: WinApiFlag = WAIT_SHARED_LOCK | RAW_LOCK_NB;
	pub const WAIT_SHARED_LOCK: WinApiFlag = 0; // 0 always (no description found for 'shared'.)

	//pub const TRY_UNLOCK: WinApiFlag			= WINDOWS UNSUPPORTED
	pub const WAIT_UNLOCK: () = ();

	// RAW
	pub const RAW_LOCK_NB: WinApiFlag = winapi::um::minwinbase::LOCKFILE_FAIL_IMMEDIATELY;
	pub const DW_RESERVED: WinApiFlag = 0; // ALWAYS 0
	                                   //
}

/// Platform number specifying platform numbers for locks.
pub type FlockRangePNum = DWORD; // DWORD = u32

impl FlockRangePNumBeh for DWORD {
	const MIN: DWORD = 0;
	const MAX: DWORD = MAXDWORD;

	#[inline(always)]
	fn get_pnum(self) -> FlockRangePNum {
		self as _
	}
}
__make_auto_pnum_type!(DWORD);

impl FlockRangePNumBeh for usize {
	const MIN: usize = 0 as _;
	const MAX: usize = MAXDWORD as _;

	#[inline(always)]
	fn get_pnum(self) -> FlockRangePNum {
		self as _
	}
}
__make_auto_pnum_type!(usize);

// TryFlockUnlock ! ....
impl<T> WaitFlockUnlock for T
where
	T: FlockElement<FilePtr = RawFilePtr>,
{
	#[inline]
	unsafe fn unlock_no_result(&mut self) {
		next_force_flock_ignore_result::<UnflockMethod, _, _, _, _, _>(
			self,
			__internal_flags::WAIT_UNLOCK,
			FlockRangeFull::<DWORD>::full(),
		)
	}

	#[inline]
	unsafe fn unlock(&mut self) -> Result<(), IoError> {
		WaitFlockUnlock::unlock_fn(self, || Ok(()), |e| Err(e))
	}

	#[inline]
	unsafe fn unlock_fn<R>(
		&mut self,
		next: impl FnOnce() -> R,
		errf: impl FnOnce(IoError) -> R,
	) -> R {
		next_force_flock::<UnflockMethod, _, _, _, _, _, _, _, _>(
			self,
			__internal_flags::WAIT_UNLOCK,
			FlockRangeFull::<DWORD>::full(),
			|_| next(),             // TODO +-
			|e| errf(e.into_err()), // TODO +-
		)
	}
}

impl<T> WaitFlockUnlockRange for T
where
	T: FlockElement<FilePtr = RawFilePtr>,
{
	#[inline]
	unsafe fn unlock_range_no_result<NS, NE, C>(&mut self, range: impl Into<FlockRange<NS, NE, C>>)
	where
		NS: FlockRangePNumBeh,
		NE: FlockRangePNumBeh + PartialOrd<NS>,
		C: FlockRangePNumBehChecker,
	{
		next_force_flock_ignore_result::<UnflockMethod, _, _, _, _, _>(
			self,
			__internal_flags::WAIT_UNLOCK,
			range,
		)
	}

	#[inline]
	unsafe fn unlock_range<NS, NE, C>(
		&mut self,
		range: impl Into<FlockRange<NS, NE, C>>,
	) -> Result<(), IoError>
	where
		NS: FlockRangePNumBeh,
		NE: FlockRangePNumBeh + PartialOrd<NS>,
		C: FlockRangePNumBehChecker,
	{
		WaitFlockUnlockRange::unlock_range_fn(self, range, || Ok(()), |e| Err(e))
	}

	#[inline]
	unsafe fn unlock_range_fn<R, NS, NE, C>(
		&mut self,
		range: impl Into<FlockRange<NS, NE, C>>,
		next: impl FnOnce() -> R,
		errf: impl FnOnce(IoError) -> R,
	) -> R
	where
		NS: FlockRangePNumBeh,
		NE: FlockRangePNumBeh + PartialOrd<NS>,
		C: FlockRangePNumBehChecker,
	{
		next_force_flock::<UnflockMethod, _, _, _, _, _, _, _, _>(
			self,
			__internal_flags::WAIT_UNLOCK,
			range,
			|_| next(),             // TODO +-
			|e| errf(e.into_err()), // TODO +-
		)
	}
}

impl<T> SharedFlock for T
where
	T: FlockElement<FilePtr = RawFilePtr>,
{
	#[inline]
	fn try_lock_fn<R>(
		self,
		next: impl FnOnce(FlockLock<Self>) -> R,
		errf: impl FnOnce(FlockError<Self>) -> R,
	) -> R {
		next_safe_flock::<FlockMethod, _, _, _, _, _, _, _, _>(
			self,
			__internal_flags::TRY_SHARED_LOCK,
			FlockRangeFull::<DWORD>::full(),
			next,
			errf,
		)
	}

	#[inline]
	fn wait_lock_fn<R>(
		self,
		next: impl FnOnce(FlockLock<Self>) -> R,
		errf: impl FnOnce(FlockError<Self>) -> R,
	) -> R {
		next_safe_flock::<FlockMethod, _, _, _, _, _, _, _, _>(
			self,
			__internal_flags::WAIT_SHARED_LOCK,
			FlockRangeFull::<DWORD>::full(),
			next,
			errf,
		)
	}
}

impl<T> ExclusiveFlock for T
where
	T: FlockElement<FilePtr = RawFilePtr>,
{
	#[inline]
	fn try_lock_fn<R>(
		self,
		next: impl FnOnce(FlockLock<Self>) -> R,
		errf: impl FnOnce(FlockError<Self>) -> R,
	) -> R {
		next_safe_flock::<FlockMethod, _, _, _, _, _, _, _, _>(
			self,
			__internal_flags::TRY_EXCLUSIVE_LOCK,
			FlockRangeFull::<DWORD>::full(),
			next,
			errf,
		)
	}

	#[inline]
	fn wait_lock_fn<R>(
		self,
		next: impl FnOnce(FlockLock<Self>) -> R,
		errf: impl FnOnce(FlockError<Self>) -> R,
	) -> R {
		next_safe_flock::<FlockMethod, _, _, _, _, _, _, _, _>(
			self,
			__internal_flags::WAIT_EXCLUSIVE_LOCK,
			FlockRangeFull::<DWORD>::full(),
			next,
			errf,
		)
	}
}

enum FlockMethod {} // default
enum UnflockMethod {}

trait CurrentFlockMethod {
	type InFlags;

	unsafe fn run(
		ptr: RawFilePtr,

		flag: Self::InFlags,
		dwReserved: DWORD,

		nNumberOfBytesToLockLow: DWORD,
		nNumberOfBytesToLockHigh: DWORD,

		lpOverlapped: LPOVERLAPPED,
	) -> RawBool;
}

impl CurrentFlockMethod for FlockMethod {
	type InFlags = __internal_flags::WinApiFlag;

	#[inline(always)]
	unsafe fn run(
		ptr: RawFilePtr,

		flag: Self::InFlags,
		dwReserved: DWORD,

		nNumberOfBytesToLockLow: DWORD,
		nNumberOfBytesToLockHigh: DWORD,

		lpOverlapped: LPOVERLAPPED,
	) -> RawBool {
		winapi::um::fileapi::LockFileEx(
			ptr,
			flag,
			dwReserved,
			nNumberOfBytesToLockLow,
			nNumberOfBytesToLockHigh,
			lpOverlapped,
		)
	}
}

impl CurrentFlockMethod for UnflockMethod {
	type InFlags = ();

	#[inline(always)]
	unsafe fn run(
		ptr: RawFilePtr,

		_flag: Self::InFlags,
		dwReserved: DWORD,

		nNumberOfBytesToLockLow: DWORD,
		nNumberOfBytesToLockHigh: DWORD,

		lpOverlapped: LPOVERLAPPED,
	) -> RawBool {
		winapi::um::fileapi::UnlockFileEx(
			ptr,
			//flag,
			dwReserved,
			nNumberOfBytesToLockLow,
			nNumberOfBytesToLockHigh,
			lpOverlapped,
		)
	}
}

#[inline(always)]
fn next_force_flock_ignore_result<
	FLM: CurrentFlockMethod,
	FE: FlockElement<FilePtr = RawFilePtr>,
	IRange: Into<FlockRange<NS, NE, C>>,
	NS: FlockRangePNumBeh,
	NE: FlockRangePNumBeh + PartialOrd<NS>,
	C: FlockRangePNumBehChecker,
>(
	data: FE,
	flag: FLM::InFlags,
	range: IRange,
) {
	let range = range.into();

	if range.is_valid_range() {
		let (start_range, end_range) = range.get();

		unsafe {
			#[allow(unused_unsafe)]
			let mut overlapped: OVERLAPPED = unsafe { core::mem::zeroed() }; // always zero, auto init hEvent

			let ptr = FlockElement::as_file_ptr(&data);
			let _result = FLM::run(
				ptr,
				flag,
				__internal_flags::DW_RESERVED,
				start_range,
				end_range,
				(&mut overlapped) as *mut _,
			);

			drop(ptr);
			drop(overlapped);
		}
	}
}

#[inline(always)]
fn next_force_flock<
	FLM: CurrentFlockMethod,
	FE: FlockElement<FilePtr = RawFilePtr>,
	IRange: Into<FlockRange<NS, NE, C>>,
	NS: FlockRangePNumBeh,
	NE: FlockRangePNumBeh + PartialOrd<NS>,
	C: FlockRangePNumBehChecker,
	N: FnOnce(FE) -> R,
	NF: FnOnce(FlockError<FE>) -> R,
	R,
>(
	data: FE,
	flag: FLM::InFlags,
	range: IRange,
	next: N,
	errf: NF,
) -> R {
	let range = range.into();

	match range.check_range_and_get() {
		Ok((start_range, end_range)) => {
			let result = unsafe {
				#[allow(unused_unsafe)]
				let mut overlapped: OVERLAPPED = unsafe { core::mem::zeroed() }; // always zero, auto init hEvent

				let ptr = FlockElement::as_file_ptr(&data);
				let result = FLM::run(
					ptr,
					flag,
					__internal_flags::DW_RESERVED,
					start_range,
					end_range,
					(&mut overlapped) as *mut _,
				);
				drop(ptr);
				drop(overlapped);

				result
			};

			match result {
				1 => next(data),
				_ => {
					#[allow(unused_mut)]
					let mut platform_err = IoError::last_os_error();

					// TODO,
					#[cfg(all(
						feature = "std",
						any(feature = "win_fix_woudblock_in_errresult", test, debug_assertions)
					))]
					if let Some(code) = platform_err.raw_os_error() {
						if code == 33 {
							drop(code);
							// TODO, FIXME, MATCH UNSTABLE!
							// TODO, FIXME, AS_STR PRIVATE..        :(
							/*if platform_err.kind().as_str() == "uncategorized error" {

							}*/

							match platform_err.get_ref() {
								Some(..) => {
									let message = match platform_err.into_inner() {
										Some(a) => a,
										None => unreachable!(),
									};

									platform_err =
										IoError::new(crate::err::IoErrorKind::WouldBlock, message);
								}
								None => {
									platform_err = IoError::new(
										crate::err::IoErrorKind::WouldBlock,
										platform_err.to_string(),
									);
								}
							}
						}
					}

					let err = FlockError::new(data, platform_err);
					errf(err)
				}
			}
		}
		Err(err_range) => errf(FlockError::new(data, err_range.make_io_error())),
	}
}

/*
	TODO, WAIT STABLE `explicit_generic_args_with_impl_trait` #83701
*/

#[inline(always)]
fn next_safe_flock<
	FLM: CurrentFlockMethod,
	FE: FlockElement<FilePtr = RawFilePtr>,
	NS: FlockRangePNumBeh,
	NE: FlockRangePNumBeh + PartialOrd<NS>,
	C: FlockRangePNumBehChecker,
	IRange: Into<FlockRange<NS, NE, C>>,
	N: FnOnce(FlockLock<FE>) -> R,
	NF: FnOnce(FlockError<FE>) -> R,
	R,
>(
	data: FE,
	flag: FLM::InFlags,
	range: IRange,
	next: N,
	errf: NF,
) -> R {
	next_force_flock::<FLM, _, _, _, _, _, _, _, _>(
		data,
		flag,
		range,
		|data| {
			let safe_flock = unsafe { FlockLock::force_new(data) };

			next(safe_flock)
		},
		errf,
	)
}
