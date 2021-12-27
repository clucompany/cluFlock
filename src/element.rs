
//! FlockElement is required to implement additional Flock locks.
//!

/// FlockElement is required to implement additional Flock locks.
pub trait FlockElement {
	/// Unix: RawFd,
	/// Win: RawHandle
	type FilePtr;
	
	fn as_file_ptr(&self) -> Self::FilePtr;
}

impl<'a, 'l, T: 'l> FlockElement for &'a T where T: FlockElement {
	type FilePtr = T::FilePtr;
	
	#[inline(always)]
	fn as_file_ptr(&self) -> Self::FilePtr {
		T::as_file_ptr(self)
	}
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
