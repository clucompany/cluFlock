//! An unsafe version of the file that consists only of a raw pointer.
//!

use crate::element::FlockElement;

/// An unsafe version of the file that consists only of a raw pointer.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct RawFile {
	ptr: crate::sys::RawFilePtr,
}

impl RawFile {
	/// Use the pointer as a primitive file for further locking.
	#[inline]
	pub const unsafe fn from_ptr(ptr: crate::sys::RawFilePtr) -> Self {
		Self { ptr }
	}

	/// Get file pointer.
	#[inline(always)]
	pub const fn get_file_ptr(&self) -> crate::sys::RawFilePtr {
		self.ptr
	}
}

impl FlockElement for RawFile {
	type FilePtr = crate::sys::RawFilePtr;

	#[inline(always)]
	fn as_file_ptr(&self) -> Self::FilePtr {
		self.ptr
	}
}

/*impl GetRawFile for RawFile {
	#[inline(always)]
	unsafe fn get_raw_file(&self) -> RawFile {
		*self // copy
	}
}*/

/// Getting the raw unmanaged file to implement flock functionality.
pub trait GetRawFile {
	/// Getting the raw unmanaged file to implement flock functionality.
	unsafe fn get_raw_file(&self) -> RawFile;
}

impl<T> GetRawFile for T
where
	T: FlockElement<FilePtr = crate::sys::RawFilePtr>,
{
	#[inline]
	unsafe fn get_raw_file(&self) -> RawFile {
		let ptr = self.as_file_ptr();

		RawFile::from_ptr(ptr)
	}
}
