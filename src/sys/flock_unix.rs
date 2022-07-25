
//! Implementation for platforms with flock support.

use crate::err::IoError;
use crate::unlock::TryFlockUnlock;
use crate::unlock::WaitFlockUnlock;
use crate::element::FlockElement;
use crate::range::pnum::FlockRangePNumBeh;
use crate::err::FlockError;
use crate::ExclusiveFlock;
use crate::FlockLock;
use crate::SharedFlock;

crate::cfg_std! {
	if #std {
		use std::fs::File;
		use std::os::unix::io::AsRawFd;
		
		use std::os::unix::io::RawFd;
		
		pub type RawFilePtr = RawFd;
		
		impl FlockElement for File {
			type FilePtr = RawFilePtr;
			
			#[inline(always)]
			fn as_file_ptr(&self) -> Self::FilePtr {
				AsRawFd::as_raw_fd(self)
			}
		}
	}else {
		pub type RawFilePtr = libc::c_int;
	}
}

pub type FlockRangePNum = usize; // unsupport :(

impl FlockRangePNumBeh for FlockRangePNum {
	const MIN: usize = 1;
	const MAX: usize = 0;
	
	#[inline(always)]
	fn get_pnum(self) -> FlockRangePNum {
		self as _
	}
}
crate::__make_auto_pnum_type!(FlockRangePNum);

mod __internal_flags {
	pub type LibcFlag = libc::c_int;

	pub const TRY_EXCLUSIVE_LOCK: LibcFlag		= WAIT_EXCLUSIVE_LOCK	| libc::LOCK_NB;
	pub const WAIT_EXCLUSIVE_LOCK: LibcFlag		= libc::LOCK_EX;

	pub const TRY_SHARED_LOCK: LibcFlag		= WAIT_SHARED_LOCK		| libc::LOCK_NB;
	pub const WAIT_SHARED_LOCK: LibcFlag		= libc::LOCK_SH;

	pub const TRY_UNLOCK: LibcFlag			= WAIT_UNLOCK			| libc::LOCK_NB;
	pub const WAIT_UNLOCK: LibcFlag			= libc::LOCK_UN;
}

impl<T> TryFlockUnlock for T where T: FlockElement<FilePtr = RawFilePtr> {
	#[inline]
	unsafe fn unlock_no_result(&mut self) {
		next_force_flock_ignore_result(
			self,
			
			__internal_flags::TRY_UNLOCK
		)
	}
	
	#[inline]
	unsafe fn unlock(&mut self) -> Result<(), IoError> {
		TryFlockUnlock::unlock_fn(
			self,
			
			|| Ok(()),
			|e| Err(e)
		)
	}
	
	#[inline]
	unsafe fn unlock_fn<R>(&mut self, next: impl FnOnce() -> R, errf: impl FnOnce(IoError) -> R) -> R {
		next_force_flock(
			self,
			__internal_flags::TRY_UNLOCK,
			
			|_| next(), // TODO +-
			|e| errf(e.into_err()) // TODO +-
		)
	}
}

impl<T> WaitFlockUnlock for T where T: FlockElement<FilePtr = RawFilePtr> {
	#[inline]
	unsafe fn unlock_no_result(&mut self) {
		next_force_flock_ignore_result(
			self,
			__internal_flags::WAIT_UNLOCK
		)
	}
	
	#[inline]
	unsafe fn unlock(&mut self) -> Result<(), IoError> {
		WaitFlockUnlock::unlock_fn(
			self,
			
			|| Ok(()),
			|e| Err(e)
		)
	}
	
	#[inline]
	unsafe fn unlock_fn<R>(&mut self, next: impl FnOnce() -> R, errf: impl FnOnce(IoError) -> R) -> R {
		next_force_flock(
			self,
			__internal_flags::WAIT_UNLOCK,
			
			|_| next(), // TODO +-
			|e| errf(e.into_err()) // TODO +-
		)
	}
}


impl<T> SharedFlock for T where T: FlockElement<FilePtr = RawFilePtr> {
	#[inline]
	fn try_lock_fn<R>(self, next: impl FnOnce(FlockLock<Self>) -> R, errf: impl FnOnce(FlockError<Self>) -> R) -> R {
		next_safe_flock(
			self,
			__internal_flags::TRY_SHARED_LOCK,
			
			next,
			errf
		)
	}
	
	#[inline]
	fn wait_lock_fn<R>(self, next: impl FnOnce(FlockLock<Self>) -> R, errf: impl FnOnce(FlockError<Self>) -> R) -> R {
		next_safe_flock(
			self,
			__internal_flags::WAIT_SHARED_LOCK,
			
			next,
			errf
		)
	}
}

impl<T> ExclusiveFlock for T where T: FlockElement<FilePtr = RawFilePtr> {
	#[inline]
	fn try_lock_fn<R>(self, next: impl FnOnce(FlockLock<Self>) -> R, errf: impl FnOnce(FlockError<Self>) -> R) -> R {
		next_safe_flock(
			self,
			__internal_flags::TRY_EXCLUSIVE_LOCK,
			
			next,
			errf
		)
	}
	
	#[inline]
	fn wait_lock_fn<R>(self, next: impl FnOnce(FlockLock<Self>) -> R, errf: impl FnOnce(FlockError<Self>) -> R) -> R {
		next_safe_flock(
			self, 
			__internal_flags::WAIT_EXCLUSIVE_LOCK,
			
			next,
			errf
		)
	}
}

#[inline(always)]
fn next_force_flock_ignore_result<D: FlockElement<FilePtr = RawFilePtr>>(data: D, flag: __internal_flags::LibcFlag) {
	unsafe {
		let ptr = FlockElement::as_file_ptr(&data);
		
		libc::flock(ptr, flag);
	}
}

#[inline(always)]
fn next_force_flock<D: FlockElement<FilePtr = RawFilePtr>, R>
	(data: D, flag: __internal_flags::LibcFlag, next: impl FnOnce(D) -> R, errf: impl FnOnce(FlockError<D>) -> R) -> R {
	let result = unsafe {
		let ptr = FlockElement::as_file_ptr(&data);
		
		libc::flock(ptr, flag)
	};
	
	match result {
		0 => next(data),
		_ => {
			let platform_err = IoError::last_os_error();
			let err = FlockError::new(data, platform_err);
			
			errf(err)
		},
	}
}

#[inline(always)]
fn next_safe_flock<D: FlockElement<FilePtr = RawFilePtr>, R>
	(data: D, flag: __internal_flags::LibcFlag, next: impl FnOnce(FlockLock<D>) -> R, errf: impl FnOnce(FlockError<D>) -> R) -> R {
	next_force_flock(
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
