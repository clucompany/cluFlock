

use crate::FlockUnlock;
use crate::raw::RawConstFlock;
use crate::FlockLock;


use std::ops::Deref;
use std::fs::File;
use std::io;

#[derive(Debug)]
pub struct FileSliceFlock<'a>(&'a File, SliceFileUnlock<'a>);
impl<'a> FlockLock for FileSliceFlock<'a> {}



impl<'a> FileSliceFlock<'a> {
	#[inline]
	const fn new(f: &'a File) -> Self {
		FileSliceFlock(f, SliceFileUnlock(f))
	}

	pub fn wait_lock_exclusive(f: &'a File) -> Result<Self, io::Error> {
		crate::sys::wait_lock_exclusive::<Self>(f)
	}

	pub fn wait_lock_shared(f: &'a File) -> Result<Self, io::Error> {
		crate::sys::wait_lock_shared::<Self>(f)
	}


	pub fn try_lock_exclusive(f: &'a File) -> Result<Self, io::Error> {
		crate::sys::try_lock_exclusive::<Self>(f)
	}


	pub fn try_lock_shared(f: &'a File) -> Result<Self, io::Error> {
		crate::sys::try_lock_shared::<Self>(f)
	}

}

impl<'a> FlockUnlock for FileSliceFlock<'a> {
	type ResultUnlock = &'a File;

	#[inline(always)]
	fn unlock(self) -> Self::ResultUnlock {
		self.into()
	}
}



impl<'a> RawConstFlock for FileSliceFlock<'a> {
	type Lock = Self;
	type Arg = &'a File;

	#[inline(always)]
	fn next(a: Self::Arg) -> Self::Lock {
		Self::Lock::new(a)
	}
}

impl<'a> Into<&'a File> for FileSliceFlock<'a> {
	#[inline(always)]
	fn into(self) -> &'a File {
		self.0
	}
}


impl<'a> Deref for FileSliceFlock<'a> {
	type Target = File;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a> AsRef<File> for FileSliceFlock<'a> {
	#[inline(always)]
	fn as_ref(&self) -> &File {
		&*self
	}
}




#[derive(Debug)]
struct SliceFileUnlock<'a>(&'a File);

impl<'a> Drop for SliceFileUnlock<'a> {
	fn drop(&mut self) {
		let _e = crate::sys::unlock(self.0);
	}
}
