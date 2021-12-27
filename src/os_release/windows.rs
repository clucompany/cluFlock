
use winapi::um::minwinbase::LPOVERLAPPED;
use winapi::shared::minwindef::BOOL;
use winapi::um::winnt::HANDLE;
use crate::data::unlock::WaitFlockUnlock;
use winapi::um::minwinbase::OVERLAPPED;
use winapi::um::winnt::MAXDWORD;
use winapi::um::fileapi::LockFileEx;
use winapi::um::fileapi::UnlockFileEx;
use crate::ExclusiveFlock;
use crate::SharedFlock;
use winapi::shared::minwindef::DWORD;
use crate::data::FlockLock;
use crate::data::err::FlockError;
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
		force_run_flock_ignore_result::<UnflockMethod, _>(
			self,
			WAIT_UNLOCK
		)
	}
	
	#[inline]
	unsafe fn unlock_fn<F: FnOnce() -> R, FE: FnOnce(std::io::Error) -> R, R>(&mut self, next: F, errf: FE) -> R {
		force_run_flock::<UnflockMethod, _, _, _, _>(
			self,
			WAIT_UNLOCK,
			|_| next(), // TODO +-
			|e| errf(e.into_err()) // TODO +-
		)
	}
}

impl<T> SharedFlock for T where T: FlockElement<FilePtr = RawHandle> {
	#[inline]
	fn try_lock_fn<F: FnOnce(FlockLock<Self>) -> R, FE: FnOnce(FlockError<Self>) -> R, R>(self, next: F, errf: FE) -> R {
		next_safe_flock::<FlockMethod, _, _, _, _>(
			self,
			TRY_SHARED_LOCK,
			next,
			errf
		)
	}
	
	#[inline]
	fn wait_lock_fn<F: FnOnce(FlockLock<Self>) -> R, FE: FnOnce(FlockError<Self>) -> R, R>(self, next: F, errf: FE) -> R {
		next_safe_flock::<FlockMethod, _, _, _, _>(
			self,
			WAIT_SHARED_LOCK,
			next,
			errf
		)
	}
}

impl<T> ExclusiveFlock for T where T: FlockElement<FilePtr = RawHandle> {
	#[inline]
	fn try_lock_fn<F: FnOnce(FlockLock<Self>) -> R, FE: FnOnce(FlockError<Self>) -> R, R>(self, next: F, errf: FE) -> R {
		next_safe_flock::<FlockMethod, _, _, _, _>(
			self,
			TRY_EXCLUSIVE_LOCK,
			next,
			errf
		)
	}
	
	#[inline]
	fn wait_lock_fn<F: FnOnce(FlockLock<Self>) -> R, FE: FnOnce(FlockError<Self>) -> R, R>(self, next: F, errf: FE) -> R {
		next_safe_flock::<FlockMethod, _, _, _, _>(
			self, 
			WAIT_EXCLUSIVE_LOCK,
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
fn force_run_flock_ignore_result<FLM: CurrentFlockMethod, D: FlockElement<FilePtr = RawHandle>>(data: D, flag: FLM::InFlags) {
	unsafe {
		#[allow(unused_unsafe)]
		let mut overlapped: OVERLAPPED = unsafe { std::mem::zeroed() }; // always zero, auto init hEvent
		
		let ptr = FlockElement::as_file_ptr(&data);
		FLM::run(
			ptr as _, 
			flag, 
			DW_RESERVED, 
			0, 
			MAXDWORD, 
			(&mut overlapped) as *mut _
		);
		
		drop(overlapped);
	}
}

#[inline(always)]
fn force_run_flock<FLM: CurrentFlockMethod, D: FlockElement<FilePtr = RawHandle>, F: FnOnce(D) -> R, FE: FnOnce(FlockError<D>) -> R, R>(data: D, flag: FLM::InFlags, next: F, errf: FE) -> R {
	let result = unsafe {
		#[allow(unused_unsafe)]
		let mut overlapped: OVERLAPPED = unsafe { std::mem::zeroed() }; // always zero, auto init hEvent
		
		let ptr = FlockElement::as_file_ptr(&data);
		FLM::run(
			ptr as _, 
			flag, 
			DW_RESERVED, 
			0, 
			MAXDWORD, 
			(&mut overlapped) as *mut _
		)
	};
	
	match result {
		1 => next(data),
		_ => {
			#[allow(unused_mut)]
			let mut platform_err = std::io::Error::last_os_error();
			
			// TODO,
			#[cfg(any(feature = "win_fix_woudblock_in_errresult", test))]
			if let Some(code) = platform_err.raw_os_error() {
				if code == 33 {
					// TODO, FIXME, MATCH UNSTABLE!
					// TODO, FIXME, AS_STR PRIVATE..        :(
					/*if platform_err.kind().as_str() == "uncategorized error" {
						
					}*/
					
					match platform_err.get_ref() {
						Some(_) => {
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

#[inline(always)]
fn next_safe_flock<FLM: CurrentFlockMethod, D: FlockElement<FilePtr = RawHandle>, F: FnOnce(FlockLock<D>) -> R, FE: FnOnce(FlockError<D>) -> R, R>(data: D, flag: FLM::InFlags, next: F, errf: FE) -> R {
	force_run_flock::<FLM, _, _, _, _>(
		data,
		flag,
		|data| {
			let safe_flock = unsafe {
				FlockLock::force_new(data)
			};
			
			next(safe_flock)
		},
		
		errf
	)
}
