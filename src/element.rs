
//! FlockElement is required to implement additional Flock locks.
//!

use crate::data::unlock::SafeUnlockFlock;
use crate::data::unlock::WaitFlockUnlock;
use crate::FlockFnBuilder;

/// FlockElement is required to implement additional Flock locks.
pub trait FlockElement {
	/// Unix: RawFd,
	/// Win: RawHandle
	type FilePtr;
	
	fn as_file_ptr(&self) -> Self::FilePtr;
}


impl<D, F, Fr> FlockElement for FlockFnBuilder<D, F, Fr> where D: FlockElement + WaitFlockUnlock, F: FnOnce(SafeUnlockFlock<D>) -> Fr {
	type FilePtr = D::FilePtr;
	
	#[inline(always)]
	fn as_file_ptr(&self) -> Self::FilePtr {
		self.data.as_file_ptr()
	}
}

impl<'a, 'l, T: 'l> FlockElement for &'a T where T: FlockElement {
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
