
//! Data types supported by FlockLock.

use core::fmt::Debug;

crate::cfg_std! {
	if #std {
		use std::boxed::Box;
	} else {
		extern crate alloc;
		use alloc::boxed::Box;
	}
}

/// FlockElement is required to implement additional 
/// Flock locks for various types of file structures.
pub trait FlockElement: Debug {
	/// Unix: RawFd,
	/// Win: RawHandle
	type FilePtr;
	
	fn as_file_ptr(&self) -> Self::FilePtr;
}

impl<T> FlockElement for Box<T> where T: FlockElement {
	type FilePtr = T::FilePtr;
	
	#[inline(always)]
	fn as_file_ptr(&self) -> Self::FilePtr {
		T::as_file_ptr(self)
	}
}

impl<'a, 'l, T: 'l> FlockElement for &'a mut T where T: FlockElement {
	type FilePtr = T::FilePtr;
	
	#[inline(always)]
	fn as_file_ptr(&self) -> Self::FilePtr {
		T::as_file_ptr(self)
	}
}

impl<'a, 'l, T: 'l> FlockElement for &'a T where T: FlockElement {
	type FilePtr = T::FilePtr;
	
	#[inline(always)]
	fn as_file_ptr(&self) -> Self::FilePtr {
		T::as_file_ptr(self)
	}
}
