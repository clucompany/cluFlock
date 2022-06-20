
use std::io::Error;
use crate::unlock::TryFlockUnlock;
use crate::unlock::WaitFlockUnlock;
use crate::element::FlockElement;
use crate::err::FlockError;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use crate::ExclusiveFlock;
use crate::FlockLock;
use crate::SharedFlock;
use std::os::unix::io::RawFd;

type LibcFlag = libc::c_int;

const TRY_EXCLUSIVE_LOCK: LibcFlag			= WAIT_EXCLUSIVE_LOCK	| libc::LOCK_NB;
const WAIT_EXCLUSIVE_LOCK: LibcFlag		= libc::LOCK_EX;

const TRY_SHARED_LOCK: LibcFlag			= WAIT_SHARED_LOCK		| libc::LOCK_NB;
const WAIT_SHARED_LOCK: LibcFlag			= libc::LOCK_SH;

const TRY_UNLOCK: LibcFlag				= WAIT_UNLOCK			| libc::LOCK_NB;
const WAIT_UNLOCK: LibcFlag				= libc::LOCK_UN;

impl FlockElement for File {
	type FilePtr = RawFd;
	
	#[inline(always)]
	fn as_file_ptr(&self) -> Self::FilePtr {
		AsRawFd::as_raw_fd(self)
	}
}

impl<T> TryFlockUnlock for T where T: FlockElement<FilePtr = RawFd> {
	#[inline]
	unsafe fn unlock_no_result(&mut self) {
		force_run_flock_ignore_result(
			self,
			TRY_UNLOCK
		)
	}
	
	#[inline]
	unsafe fn unlock_fn<R>(&mut self, next: impl FnOnce() -> R, errf: impl FnOnce(Error) -> R) -> R {
		force_run_flock(
			self,
			TRY_UNLOCK,
			|_| next(), // TODO +-
			|e| errf(e.into_err()) // TODO +-
		)
	}
}

impl<T> WaitFlockUnlock for T where T: FlockElement<FilePtr = RawFd> {
	#[inline]
	unsafe fn unlock_no_result(&mut self) {
		force_run_flock_ignore_result(
			self,
			WAIT_UNLOCK
		)
	}
	
	#[inline]
	unsafe fn unlock_fn<R>(&mut self, next: impl FnOnce() -> R, errf: impl FnOnce(Error) -> R) -> R {
		force_run_flock(
			self,
			WAIT_UNLOCK,
			|_| next(), // TODO +-
			|e| errf(e.into_err()) // TODO +-
		)
	}
}


impl<T> SharedFlock for T where T: FlockElement<FilePtr = RawFd> {
	#[inline]
	fn try_lock_fn<R>(self, next: impl FnOnce(FlockLock<Self>) -> R, errf: impl FnOnce(FlockError<Self>) -> R) -> R {
		next_safe_flock(
			self,
			TRY_SHARED_LOCK,
			next,
			errf
		)
	}
	
	#[inline]
	fn wait_lock_fn<R>(self, next: impl FnOnce(FlockLock<Self>) -> R, errf: impl FnOnce(FlockError<Self>) -> R) -> R {
		next_safe_flock(
			self,
			WAIT_SHARED_LOCK,
			next,
			errf
		)
	}
}

impl<T> ExclusiveFlock for T where T: FlockElement<FilePtr = RawFd> {
	#[inline]
	fn try_lock_fn<R>(self, next: impl FnOnce(FlockLock<Self>) -> R, errf: impl FnOnce(FlockError<Self>) -> R) -> R {
		next_safe_flock(
			self,
			TRY_EXCLUSIVE_LOCK,
			next,
			errf
		)
	}
	
	#[inline]
	fn wait_lock_fn<R>(self, next: impl FnOnce(FlockLock<Self>) -> R, errf: impl FnOnce(FlockError<Self>) -> R) -> R {
		next_safe_flock(
			self, 
			WAIT_EXCLUSIVE_LOCK,
			next,
			errf
		)
	}
}

#[inline(always)]
fn force_run_flock_ignore_result<D: FlockElement<FilePtr = RawFd>>(data: D, flag: LibcFlag) {
	unsafe {
		libc::flock(FlockElement::as_file_ptr(&data), flag);
	}
}

#[inline(always)]
fn force_run_flock<D: FlockElement<FilePtr = RawFd>, F: FnOnce(D) -> R, FE: FnOnce(FlockError<D>) -> R, R>(data: D, flag: LibcFlag, next: F, errf: FE) -> R {
	let result = unsafe {
		let ptr = FlockElement::as_file_ptr(&data);
		
		libc::flock(ptr, flag)
	};
	
	match result {
		0 => next(data),
		_ => {
			let platform_err = std::io::Error::last_os_error();
			let err = FlockError::new(data, platform_err);
			
			errf(err)
		},
	}
}


#[inline(always)]
fn next_safe_flock<D: FlockElement<FilePtr = RawFd>, F: FnOnce(FlockLock<D>) -> R, FE: FnOnce(FlockError<D>) -> R, R>(data: D, flag: LibcFlag, next: F, errf: FE) -> R {
	force_run_flock(
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
