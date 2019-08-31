

use std::ops::DerefMut;
use std::ops::Deref;
use std::io;

/// Generic describing the function of destroying 'flock' locks.
pub trait FlockUnlock {
	type UnlockResult;
	// Default: ()
	
	/// Destroy the 'flock 'lock without checking for errors, this function is used in Drop.
	unsafe fn flock_unlock_no_result(&mut self);
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn flock_unlock(&mut self) -> Result<Self::UnlockResult, io::Error>;
	
	// Why initially unsafe?
	// 
	// The structure should be destroyed after calling these functions, 
	// but since Drop `&mut self`; we cannot guarantee this outside our library.
	//
}


/// The structure that controls the 'flock' lock.
#[derive(Debug)]
pub struct SafeUnlockFlock<T> where T: FlockUnlock {
	data: T,
}

#[cfg(feature = "nightly")]
#[allow(non_camel_case_types)]
struct __SafeUnlockFlock_DropData<T> where T: FlockUnlock {
	data: T,	
}

impl<T> SafeUnlockFlock<T> where T: FlockUnlock {
	/// Create lock surveillance structure, unsafe because it 
	/// is not known if a lock has been created before.
	#[inline]
	pub unsafe fn new(t: T) -> Self {
		Self {
			data: t,
		}
	}
	
	/// Destroy the 'flock' lock, return a good result or error.
	pub fn unlock(mut self) -> Result<T::UnlockResult, std::io::Error> {
		let r = unsafe { self.data.flock_unlock() };
		
		//
		unsafe {
			std::ptr::drop_in_place(&mut self.data);
		}
		
		std::mem::forget(self);
		//
		
		// Why?
		// The fact is that 'mem::forget' also excludes 
		// nested destructors, so they need to be called manually.
		
		r
	}
	
	/// Destroy the "flock" lock without returning the result or error.
	pub fn unlock_no_result(mut self) {
		unsafe { self.data.flock_unlock_no_result() };
		
		//
		unsafe {
			std::ptr::drop_in_place(&mut self.data);
		}
		
		std::mem::forget(self);
		//
		
		// Why?
		// The fact is that 'mem::forget' also excludes 
		// nested destructors, so they need to be called manually.
	}
	
	
	
	/// Destroy the "flock" lock, return data and error data.
	#[cfg(feature = "nightly")]
	pub fn data_unlock(mut self) -> (T, Result<T::UnlockResult, std::io::Error>) {
		let r = unsafe { self.data.flock_unlock() };
		
		let new_self = std::mem::ManuallyDrop::new(self);
		let data: __SafeUnlockFlock_DropData<T> = unsafe { cluFullTransmute::mem::full_transmute(new_self) };
		
		(data.data, r)
	}
	
	/// Destroy the "flock" lock, return data.
	#[cfg(feature = "nightly")]
	pub fn data_unlock_no_err_result(mut self) -> T {
		let _r = unsafe { self.data.flock_unlock_no_result() };
		
		let new_self = std::mem::ManuallyDrop::new(self);
		let data: __SafeUnlockFlock_DropData<T> = unsafe { cluFullTransmute::mem::full_transmute(new_self) };
		
		data.data
	}
}



impl<T> AsRef<T> for SafeUnlockFlock<T> where T: FlockUnlock {
	#[inline(always)]
	fn as_ref(&self) -> &T {
		&self.data
	}
}
impl<T> AsMut<T> for SafeUnlockFlock<T> where T: FlockUnlock {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut T {
		&mut self.data
	}
}


impl<T> Deref for SafeUnlockFlock<T> where T: FlockUnlock {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<T> DerefMut for SafeUnlockFlock<T> where T: FlockUnlock {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut T {
		&mut self.data
	}
}

impl<T> Drop for SafeUnlockFlock<T> where T: FlockUnlock {
	#[inline(always)] // 1: fn -> fn
	fn drop(&mut self) {
		unsafe{ self.data.flock_unlock_no_result() }
	}
}
