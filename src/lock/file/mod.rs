
mod buf;
mod slice;

pub use self::buf::*;
pub use self::slice::*;

use crate::SharedFlock;
use crate::ExclusiveFlock;
use std::fs::File;
use std::io;


impl ExclusiveFlock for File {
	type ExclusiveLock = FileFlock;

	#[inline(always)]
	fn wait_lock(self) -> Result<Self::ExclusiveLock, io::Error> {
		Self::ExclusiveLock::wait_lock_exclusive(self)
	}
	
	#[inline(always)]
	fn try_lock(self) -> Result<Self::ExclusiveLock, io::Error> {
		Self::ExclusiveLock::try_lock_exclusive(self)
	}
}


impl<'a> ExclusiveFlock for &'a File {
	type ExclusiveLock = FileSliceFlock<'a>;

	#[inline(always)]
	fn wait_lock(self) -> Result<Self::ExclusiveLock, io::Error> {
		Self::ExclusiveLock::wait_lock_exclusive(self)
	}

	#[inline(always)]
	fn try_lock(self) -> Result<Self::ExclusiveLock, io::Error> {
		Self::ExclusiveLock::try_lock_exclusive(self)
	}
}



impl<'a> ExclusiveFlock for &'a mut File {
	type ExclusiveLock = FileSliceFlock<'a>;

	#[inline(always)]
	fn wait_lock(self) -> Result<Self::ExclusiveLock, io::Error> {
		Self::ExclusiveLock::wait_lock_exclusive(self)
	}

	#[inline(always)]
	fn try_lock(self) -> Result<Self::ExclusiveLock, io::Error> {
		Self::ExclusiveLock::try_lock_exclusive(self)
	}
}


//SHARED

impl SharedFlock for File {
	type SharedLock = FileFlock;

	#[inline(always)]
	fn wait_lock(self) -> Result<Self::SharedLock, io::Error> {
		Self::SharedLock::wait_lock_exclusive(self)
	}

	#[inline(always)]
	fn try_lock(self) -> Result<Self::SharedLock, io::Error> {
		Self::SharedLock::try_lock_exclusive(self)
	}
}


impl<'a> SharedFlock for &'a File {
	type SharedLock = FileSliceFlock<'a>;

	#[inline(always)]
	fn wait_lock(self) -> Result<Self::SharedLock, io::Error> {
		Self::SharedLock::wait_lock_exclusive(self)
	}
	
	#[inline(always)]
	fn try_lock(self) -> Result<Self::SharedLock, io::Error> {
		Self::SharedLock::try_lock_exclusive(self)
	}
}

impl<'a> SharedFlock for &'a mut File {
	type SharedLock = FileSliceFlock<'a>;

	#[inline(always)]
	fn wait_lock(self) -> Result<Self::SharedLock, io::Error> {
		Self::SharedLock::wait_lock_exclusive(self)
	}
	
	#[inline(always)]
	fn try_lock(self) -> Result<Self::SharedLock, io::Error> {
		Self::SharedLock::try_lock_exclusive(self)
	}
}

