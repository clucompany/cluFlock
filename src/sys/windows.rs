
use std::io::Error;
use core::ops::Range;
use crate::range::RangeFlock;
use winapi::um::minwinbase::LPOVERLAPPED;
use winapi::shared::minwindef::BOOL;
use winapi::um::winnt::HANDLE;
use crate::unlock::WaitFlockUnlock;
use winapi::um::minwinbase::OVERLAPPED;
use winapi::um::winnt::MAXDWORD;
use winapi::um::fileapi::LockFileEx;
use winapi::um::fileapi::UnlockFileEx;
use crate::ExclusiveFlock;
use crate::SharedFlock;
use winapi::shared::minwindef::DWORD;
use crate::FlockLock;
use crate::err::FlockError;
use std::fs::File;
use crate::element::FlockElement;
use std::os::windows::io::AsRawHandle;

#[cfg(any(feature = "win_fix_woudblock_in_errresult", test))]
use std::io::ErrorKind;

type RawHandle = HANDLE;
type WinApiFlag = winapi::shared::minwindef::DWORD;

const TRY_EXCLUSIVE_LOCK: WinApiFlag		= WAIT_EXCLUSIVE_LOCK	| RAW_LOCK_NB;
const WAIT_EXCLUSIVE_LOCK: WinApiFlag		= winapi::um::minwinbase::LOCKFILE_EXCLUSIVE_LOCK;

const TRY_SHARED_LOCK: WinApiFlag			= WAIT_SHARED_LOCK		| RAW_LOCK_NB;
const WAIT_SHARED_LOCK: WinApiFlag			= 0; // 0 always

//const TRY_UNLOCK: WinApiFlag			= WINDOWS UNSUPPORTED
const WAIT_UNLOCK: ()					= ();

// RAW
const RAW_LOCK_NB: WinApiFlag = winapi::um::minwinbase::LOCKFILE_FAIL_IMMEDIATELY;
const DW_RESERVED: DWORD = 0; // ALWAYS 0
//

const MIN_RANGE: DWORD = 0;
const MAX_RANGE: DWORD = MAXDWORD;
const FULL_RANGE: Range<DWORD> = Range {
	start: MIN_RANGE,
	end: MAX_RANGE,
};

/// YOU_X..=MAX_RANGE
#[derive(Debug, Clone, Copy)]
pub struct RangeStartFlock(DWORD);

impl RangeFlock for RangeStartFlock {
	type NumType = DWORD;
	type FinalTransform = Range<DWORD>;
	
	#[inline(always)]
	fn is_valid(&self) -> bool {
		true
	}
	
	#[inline(always)]
	fn final_transform(self) -> Self::FinalTransform {
		Range {
			start: self.0,
			end: MAX_RANGE,
		}
	}
	
	#[inline(always)]
	fn get_range<R>(self, _next: impl FnOnce(Self::NumType, Self::NumType) -> R) -> R {
		unimplemented!("This code should never be called and is required as a stub.");
	}
}

/// MIN_RANGE..=YOU_X
#[derive(Debug, Clone, Copy)]
pub struct RangeEndFlock(DWORD);

impl RangeFlock for RangeEndFlock {
	type NumType = DWORD;
	type FinalTransform = Range<DWORD>;
	
	#[inline(always)]
	fn is_valid(&self) -> bool {
		true
	}
	
	#[inline(always)]
	fn final_transform(self) -> Self::FinalTransform {
		Range {
			start: MIN_RANGE,
			end: MAXDWORD,
		}
	}
	
	#[inline(always)]
	fn get_range<R>(self, _next: impl FnOnce(Self::NumType, Self::NumType) -> R) -> R {
		unimplemented!("This code should never be called and is required as a stub.");
	}
}

impl RangeFlock for (DWORD, DWORD) {
	type NumType = DWORD;
	type FinalTransform = Range<DWORD>;
	
	#[inline(always)]
	fn is_valid(&self) -> bool {
		true
	}
	
	#[inline(always)]
	fn final_transform(self) -> Self::FinalTransform {
		Range {
			start: self.0,
			end: self.1,
		}
	}
	
	#[inline(always)]
	fn get_range<R>(self, _next: impl FnOnce(Self::NumType, Self::NumType) -> R) -> R {
		unimplemented!("This code should never be called and is required as a stub.");
	}
}

impl RangeFlock for Range<DWORD> {
	type NumType = DWORD;
	type FinalTransform = Self;
	
	#[inline(always)]
	fn is_valid(&self) -> bool {
		return 
			self.end >= self.start &&
			
			self.start >= MIN_RANGE && self.start <= MAX_RANGE &&
			self.end >= MIN_RANGE && self.end <= MAX_RANGE;
	}
	
	#[inline(always)]
	fn final_transform(self) -> Self::FinalTransform {
		self
	}
	
	#[inline(always)]
	fn get_range<R>(self, next: impl FnOnce(DWORD, DWORD) -> R) -> R {
		next(self.start, self.end)
	}
}

trait CheckRangeAtRuntime {
	const IS: bool;
}

enum AlwaysCheckRange {}

impl CheckRangeAtRuntime for AlwaysCheckRange {
	const IS: bool = true;
}

enum AlwaysIgnoreRangeCheckAtRuntime {}

impl CheckRangeAtRuntime for AlwaysIgnoreRangeCheckAtRuntime {
	const IS: bool = false;
}

impl FlockElement for File {
	type FilePtr = RawHandle;
	
	#[inline(always)]
	fn as_file_ptr(&self) -> Self::FilePtr {
		AsRawHandle::as_raw_handle(self) as _
	}
}


// TryFlockUnlock ! ....
impl<T> WaitFlockUnlock for T where T: FlockElement<FilePtr = RawHandle> {
	#[inline]
	unsafe fn unlock_no_result(&mut self) {
		force_run_flock_ignore_result::<UnflockMethod, _, AlwaysIgnoreRangeCheckAtRuntime, _>(
			self,
			WAIT_UNLOCK,
			FULL_RANGE,
		)
	}
	
	#[inline]
	unsafe fn unlock_fn<R>(&mut self, next: impl FnOnce() -> R, errf: impl FnOnce(Error) -> R) -> R {
		force_run_flock::<UnflockMethod, _, AlwaysIgnoreRangeCheckAtRuntime, _, _, _, _>(
			self,
			WAIT_UNLOCK,
			FULL_RANGE,
			
			|_| next(), // TODO +-
			|e| errf(e.into_err()) // TODO +-
		)
	}
}

impl<T> SharedFlock for T where T: FlockElement<FilePtr = RawHandle> {
	#[inline]
	fn try_lock_fn<R>(self, next: impl FnOnce(FlockLock<Self>) -> R, errf: impl FnOnce(FlockError<Self>) -> R) -> R {
		next_safe_flock::<FlockMethod, _, AlwaysIgnoreRangeCheckAtRuntime, _, _, _, _>(
			self,
			TRY_SHARED_LOCK,
			FULL_RANGE,
			
			next,
			errf
		)
	}
	
	#[inline]
	fn wait_lock_fn<R>(self, next: impl FnOnce(FlockLock<Self>) -> R, errf: impl FnOnce(FlockError<Self>) -> R) -> R {
		next_safe_flock::<FlockMethod, _, AlwaysIgnoreRangeCheckAtRuntime, _, _, _, _>(
			self,
			WAIT_SHARED_LOCK,
			FULL_RANGE,
			
			next,
			errf
		)
	}
}

impl<T> ExclusiveFlock for T where T: FlockElement<FilePtr = RawHandle> {
	#[inline]
	fn try_lock_fn<R>(self, next: impl FnOnce(FlockLock<Self>) -> R, errf: impl FnOnce(FlockError<Self>) -> R) -> R {
		next_safe_flock::<FlockMethod, _, AlwaysIgnoreRangeCheckAtRuntime, _, _, _, _>(
			self,
			TRY_EXCLUSIVE_LOCK,
			FULL_RANGE,
			
			next,
			errf
		)
	}
	
	#[inline]
	fn wait_lock_fn<R>(self, next: impl FnOnce(FlockLock<Self>) -> R, errf: impl FnOnce(FlockError<Self>) -> R) -> R {
		next_safe_flock::<FlockMethod, _, AlwaysIgnoreRangeCheckAtRuntime, _, _, _, _>(
			self, 
			WAIT_EXCLUSIVE_LOCK,
			FULL_RANGE,
			
			next,
			errf
		)
	}
}


enum FlockMethod {} // default
enum UnflockMethod {}

trait CurrentFlockMethod {
	type InFlags;
	
	unsafe fn run(
		ptr: HANDLE,
		flag: Self::InFlags,
		dwReserved: DWORD,
		nNumberOfBytesToLockLow: DWORD,
		nNumberOfBytesToLockHigh: DWORD,
		lpOverlapped: LPOVERLAPPED
	) -> BOOL;
}

impl CurrentFlockMethod for FlockMethod {
	type InFlags = WinApiFlag;
	
	#[inline(always)]
	unsafe fn run(
		ptr: HANDLE,
		flag: Self::InFlags,
		dwReserved: DWORD,
		nNumberOfBytesToLockLow: DWORD,
		nNumberOfBytesToLockHigh: DWORD,
		lpOverlapped: LPOVERLAPPED
	) -> BOOL {
		LockFileEx(
			ptr, 
			flag, 
			dwReserved, 
			nNumberOfBytesToLockLow, 
			nNumberOfBytesToLockHigh, 
			lpOverlapped
		)
	}
}

impl CurrentFlockMethod for UnflockMethod {
	type InFlags = ();
	
	#[inline(always)]
	unsafe fn run(
		ptr: HANDLE,
		_flag: Self::InFlags,
		dwReserved: DWORD,
		nNumberOfBytesToLockLow: DWORD,
		nNumberOfBytesToLockHigh: DWORD,
		lpOverlapped: LPOVERLAPPED
	) -> BOOL {
		UnlockFileEx(
			ptr, 
			//flag, 
			dwReserved, 
			nNumberOfBytesToLockLow, 
			nNumberOfBytesToLockHigh, 
			lpOverlapped
		)
	}
}


#[inline(always)]
fn force_run_flock_ignore_result<
	FLM: CurrentFlockMethod, 
	Range: RangeFlock<NumType = DWORD>,
	IsCheckRange: CheckRangeAtRuntime,
	
	D: FlockElement<FilePtr = RawHandle>
>(data: D, flag: FLM::InFlags, range: Range) {
	let range = range.final_transform();
	
	if IsCheckRange::IS {
		#[cfg(feature = "always_check_ranges")] {
			if !range.is_valid() {
				return;
			}
		} /*else*/
		#[cfg(not(feature = "always_check_ranges"))] {
			debug_assert_eq!(range.is_valid(), true);
		}
	}else {
		debug_assert_eq!(range.is_valid(), true);
	}
	
	unsafe {
		#[allow(unused_unsafe)]
		let mut overlapped: OVERLAPPED = unsafe { std::mem::zeroed() }; // always zero, auto init hEvent
		
		let ptr = FlockElement::as_file_ptr(&data);
		range.get_range(|start_range, end_range| {
			FLM::run(
				ptr as _, 
				flag, 
				DW_RESERVED, 
				start_range, 
				end_range, 
				(&mut overlapped) as *mut _
			);
		});
		
		drop(ptr);
		drop(overlapped);
	}
}

#[inline(always)]
fn force_run_flock<
	FLM: CurrentFlockMethod,
	Range: RangeFlock<NumType = DWORD>,
	IsCheckRange: CheckRangeAtRuntime,
	
	D: FlockElement<FilePtr = RawHandle>,
	
	F: FnOnce(D) -> R,
	FE: FnOnce(FlockError<D>) -> R, R
>(data: D, flag: FLM::InFlags, range: Range, next: F, errf: FE) -> R {
	let range = range.final_transform();
	
	if IsCheckRange::IS {
		#[cfg(feature = "always_check_ranges")] {
			if !range.is_valid() {
				let err = FlockError::new(data, std::io::Error::new(
					ErrorKind::WouldBlock,
					format!("Range not supported, {:?}", range)
				));
				return errf(err);
			}
		} /*else*/
		#[cfg(not(feature = "always_check_ranges"))] {
			debug_assert_eq!(range.is_valid(), true);
		}
	}else {
		debug_assert_eq!(range.is_valid(), true);
	}
	
	
	let result = unsafe {
		#[allow(unused_unsafe)]
		let mut overlapped: OVERLAPPED = unsafe { std::mem::zeroed() }; // always zero, auto init hEvent
		
		let ptr = FlockElement::as_file_ptr(&data);
		let result = range.get_range(|start_range, end_range| {
			FLM::run(
				ptr as _, 
				flag, 
				DW_RESERVED, 
				start_range, 
				end_range, 
				(&mut overlapped) as *mut _
			)
		});
		drop(ptr);
		drop(overlapped);
		
		result
	};
	
	match result {
		1 => next(data),
		_ => {
			#[allow(unused_mut)]
			let mut platform_err = std::io::Error::last_os_error();
			
			// TODO,
			#[cfg(any(feature = "win_fix_woudblock_in_errresult", test, debug_assertions))]
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
							
							platform_err = std::io::Error::new(
								ErrorKind::WouldBlock,
								message
							);
						},
						None => {
							platform_err = std::io::Error::new(
								ErrorKind::WouldBlock,
								platform_err.to_string()
							);
						}
					}
				}
			}
			
			let err = FlockError::new(data, platform_err);
			errf(err)
		},
	}
}

/*
	TODO, WAIT STABLE `explicit_generic_args_with_impl_trait` #83701 
*/

#[inline(always)]
fn next_safe_flock<
	FLM: CurrentFlockMethod, 
	Range: RangeFlock<NumType = DWORD>, 
	IsCheckRange: CheckRangeAtRuntime,
	
	D: FlockElement<FilePtr = RawHandle>, 
	
	F: FnOnce(FlockLock<D>) -> R, 
	FE: FnOnce(FlockError<D>) -> R, R
	
>(data: D, flag: FLM::InFlags, range: Range, next: F, errf: FE) -> R {
	force_run_flock::<FLM, _, IsCheckRange, _, _, _, _>(
		data,
		flag,
		range,
		
		|data| {
			let safe_flock = unsafe {
				FlockLock::force_new(data)
			};
			
			next(safe_flock)
		},
		
		errf
	)
}
