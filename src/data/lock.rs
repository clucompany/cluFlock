
use core::fmt::Debug;
use core::mem::ManuallyDrop;
use core::hash::Hash;
use crate::element::FlockElement;
use crate::data::unlock::WaitFlockUnlock;
use core::ops::DerefMut;
use core::ops::Deref;
use crate::err::FlockError;
use crate::ExclusiveFlock;
use crate::SharedFlock;

/// Type for securely creating and securely managing 'flock' locks.
#[derive(/*Copy, */Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FlockLock<T> where T: FlockElement + WaitFlockUnlock {
	data: ManuallyDrop<T>,
}

impl<T> Debug for FlockLock<T> where T: Debug + FlockElement + WaitFlockUnlock {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		f.debug_struct("FlockLock")
			.field("data", &self.data as &T)
			.finish()
	}
}

impl<T> FlockLock<T> where T: FlockElement + WaitFlockUnlock {
	/// Create lock surveillance structure, unsafe because it 
	/// is not known if a lock has been created before.
	#[deprecated(since="1.2.6", note="please use `force_new` instead")]
	#[inline]
	pub unsafe fn new(t: T) -> Self {
		Self::force_new(t)
	}
	
	/// Create lock surveillance structure, unsafe because it 
	/// is not known if a lock has been created before.
	#[inline]
	pub unsafe fn force_new(data: T) -> Self {
		Self {
			data: ManuallyDrop::new(data)
		}
	}
	
	//safe new
	#[inline(always)]
	pub fn wait_exclusive_lock(data: T) -> Result<FlockLock<T>, FlockError<T>> where T: ExclusiveFlock {
		ExclusiveFlock::wait_lock(data)
	}
	
	#[inline(always)]
	pub fn try_exclusive_lock(data: T) -> Result<FlockLock<T>, FlockError<T>> where T: ExclusiveFlock {
		ExclusiveFlock::try_lock(data)
	}
	
	#[inline(always)]
	pub fn wait_exclusive_lock_fn<F: FnOnce(FlockLock<T>) -> R, FE: FnOnce(FlockError<T>) -> R, R>(data: T, next: F, errf: FE) -> R where T: ExclusiveFlock {
		ExclusiveFlock::wait_lock_fn(data, next, errf)
	}
	
	#[inline(always)]
	pub fn try_exclusive_lock_fn<F: FnOnce(FlockLock<T>) -> R, FE: FnOnce(FlockError<T>) -> R, R>(data: T, next: F, errf: FE) -> R where T: ExclusiveFlock {
		ExclusiveFlock::try_lock_fn(data, next, errf)
	}


	#[inline(always)]
	pub fn wait_shared_lock(f: T) -> Result<FlockLock<T>, FlockError<T>> where T: SharedFlock {
		SharedFlock::wait_lock(f)
	}
	
	#[inline(always)]
	pub fn try_shared_lock(f: T) -> Result<FlockLock<T>, FlockError<T>> where T: SharedFlock {
		SharedFlock::try_lock(f)
	}
	
	#[inline(always)]
	pub fn wait_shared_lock_fn<F: FnOnce(FlockLock<T>) -> R, FE: FnOnce(FlockError<T>) -> R, R>(data: T, next: F, errf: FE) -> R where T: SharedFlock {
		SharedFlock::wait_lock_fn(data, next, errf)
	}
	#[inline(always)]
	pub fn try_shared_lock_fn<F: FnOnce(FlockLock<T>) -> R, FE: FnOnce(FlockError<T>) -> R, R>(data: T, next: F, errf: FE) -> R where T: SharedFlock {
		SharedFlock::try_lock_fn(data, next, errf)
	}
	//
	
	#[inline(always)]
	pub fn as_ptr(&self) -> *const T {
		// exp stable ManuallyDrop::as_ptr
		&*self.data as _
	}
	
	#[inline(always)]
	pub fn as_mut_ptr(&mut self) -> *mut T {
		// exp stable ManuallyDrop::as_mut_ptr
		&mut *self.data as _
	}
	
	/// Destroy the 'flock' lock, return a good result or error.
	#[inline]
	pub fn unlock_fn<F: FnOnce() -> R, FE: FnOnce(std::io::Error) -> R, R>(mut self, next: F, errf: FE) -> R {
		let result = unsafe {
			WaitFlockUnlock::unlock_fn(&mut *self.data, next, errf)
		};
		
		// always drop
		unsafe {
			ManuallyDrop::drop(&mut self.data);
		}
		let _ignore_self_drop = ManuallyDrop::new(self);
		
		result
	}
	
	/// Destroy the 'flock' lock, return a good result or error.
	#[inline]
	pub fn unlock(self) -> Result<(), std::io::Error> {
		self.unlock_fn(
			|| Ok(()),
			|e| Err(e)
		)
	}
	
	/// Destroy the 'flock' lock, return a good result or error.
	#[inline]
	pub fn unlock_no_err_result(mut self) {
		unsafe {
			WaitFlockUnlock::unlock_no_result(&mut *self.data);
		}
		
		// always drop
		unsafe {
			ManuallyDrop::drop(&mut self.data);
		}
		let _ignore_self_drop = ManuallyDrop::new(self);
	}
	
	/// Destroy the "flock" lock, return data and error data.
	#[inline]
	pub fn data_unlock(self) -> (T, Result<(), std::io::Error>) {
		self.data_unlock_fn(
			|| Ok(()),
			|e| Err(e)
		)
	}
	
	/// Destroy the "flock" lock, return data and error data.
	#[inline]
	pub fn data_unlock_fn<F: FnOnce() -> R, FE: FnOnce(std::io::Error) -> R, R>(mut self, next: F, errf: FE) -> (T, R) {
		let result = unsafe {
			WaitFlockUnlock::unlock_fn(&mut *self.data, next, errf)
		};
		
		//
		let data = unsafe {
			ManuallyDrop::take(&mut self.data)
		};
		let _ignore_self_drop = ManuallyDrop::new(self);
		
		(data, result)
	}
	
	
	/// Destroy the "flock" lock, return data.
	#[inline]
	pub fn data_unlock_no_err_result(mut self) -> T {
		let _result = unsafe {
			WaitFlockUnlock::unlock_no_result(&mut *self.data)
		};
		
		//
		let data = unsafe {
			ManuallyDrop::take(&mut self.data)
		};
		let _ignore_self_drop = ManuallyDrop::new(self);
		
		data
	}
}

impl<T> AsRef<T> for FlockLock<T> where T: FlockElement + WaitFlockUnlock {
	#[inline(always)]
	fn as_ref(&self) -> &T {
		&self.data
	}
}

impl<T> AsMut<T> for FlockLock<T> where T: FlockElement + WaitFlockUnlock {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut T {
		&mut self.data
	}
}

impl<T> Deref for FlockLock<T> where T: FlockElement + WaitFlockUnlock {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.data.deref()
	}
}

impl<T> DerefMut for FlockLock<T> where T: FlockElement + WaitFlockUnlock {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.data.deref_mut()
	}
}

impl<T> Drop for FlockLock<T> where T: FlockElement + WaitFlockUnlock {
	#[inline(always)]
	fn drop(&mut self) {
		unsafe {
			self.unlock_no_result();
		}
	}
}
