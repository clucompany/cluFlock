
use crate::data::unlock::WaitFlockUnlock;
use winapi::um::minwinbase::OVERLAPPED;
use winapi::um::winnt::MAXDWORD;
use winapi::um::fileapi::LockFileEx;
use winapi::um::fileapi::UnlockFileEx;
use crate::ExclusiveFlock;
use crate::SharedFlock;
use crate::os_release::r#dyn::BehOsRelease;
use crate::data::err::FlockFnError;
use winapi::shared::minwindef::DWORD;
use crate::SafeUnlockFlock;
use crate::data::FlockLock;
use crate::data::err::FlockError;
use std::os::windows::io::RawHandle;
use std::fs::File;
use crate::element::FlockElement;
use std::os::windows::io::AsRawHandle;
use std::io;
use crate::FlockFnBuilder;


const LOCK_SH: DWORD = 0;
const LOCK_EX: DWORD = winapi::um::minwinbase::LOCKFILE_EXCLUSIVE_LOCK;
const LOCK_NB: DWORD = winapi::um::minwinbase::LOCKFILE_FAIL_IMMEDIATELY;

const DW_RESERVED: DWORD = 0;

impl FlockElement for File {
	type FilePtr = RawHandle;
	
	#[inline(always)]
	fn as_file_ptr(&self) -> Self::FilePtr {
		AsRawHandle::as_raw_handle(self)
	}
}


// TryFlockUnlock ! ....

impl<T> WaitFlockUnlock for T where T: FlockElement<FilePtr = RawHandle> {
	type UnlockResult = ();

	unsafe fn unlock_no_result(&mut self) {
		let overlapped: OVERLAPPED = std::mem::zeroed();
		UnlockFileEx(self.as_file_ptr() as _, DW_RESERVED, 0, MAXDWORD, (&overlapped) as *const _ as *mut _);
	}
	
	unsafe fn unlock(&mut self) -> Result<(), io::Error> {
		let overlapped: OVERLAPPED = std::mem::zeroed();
		let result = UnlockFileEx(self.as_file_ptr() as _, DW_RESERVED, 0, MAXDWORD, (&overlapped) as *const _ as *mut _);
		
		match result {
			1 => Ok(()),
			_ => Err( io::Error::last_os_error() ),
		}
	}
}

impl<T> SharedFlock for T where T: FlockElement<FilePtr = RawHandle> {	
	fn try_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		flock::<FlockLock<Self>>(self, LOCK_SH | LOCK_NB)
	}
	fn wait_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		flock::<FlockLock<Self>>(self, LOCK_SH)
	}
	
	fn try_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> {
		flock::<FlockFnBuilder<Self, Fn, Fr>>(FlockFnBuilder::new(self, f), LOCK_SH | LOCK_NB)
	}
	fn wait_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> {
		flock::<FlockFnBuilder<Self, Fn, Fr>>(FlockFnBuilder::new(self, f), LOCK_SH)
	}
}

impl<T> ExclusiveFlock for T where T: FlockElement<FilePtr = RawHandle> {
	fn try_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		flock::<FlockLock<Self>>(self, LOCK_EX | LOCK_NB)
	}
	fn wait_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> {
		flock::<FlockLock<Self>>(self, LOCK_EX)
	}
	
	fn try_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> {
		flock::<FlockFnBuilder<Self, Fn, Fr>>(FlockFnBuilder::new(self, f), LOCK_EX | LOCK_NB)
	}
	fn wait_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> {
		flock::<FlockFnBuilder<Self, Fn, Fr>>(FlockFnBuilder::new(self, f), LOCK_EX)
	}
}


fn flock<D: FlockElement<FilePtr = RawFd>, F: FnOnce(D) -> R, FE: FnOnce(D, std::io::Error) -> R, R>(arg: D, flag: DWORD, next: F, errf: FE) -> R {
	let overlapped: OVERLAPPED = unsafe { std::mem::zeroed() };
	
	match unsafe { LockFileEx(arg.as_file_ptr() as _, flock_args, DW_RESERVED, 0, MAXDWORD, (&overlapped) as *const _ as *mut _) } {
		1 => Ok( I::ok(arg) ),
		_ => Err( I::err(arg, io::Error::last_os_error()) ),
	}
}

