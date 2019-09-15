
use crate::data::unlock::WaitFlockUnlock;
use crate::data::err::FlockFnError;
use crate::SafeUnlockFlock;
use std::ops::DerefMut;
use std::ops::Deref;
use crate::BehOsRelease;
use crate::err::FlockError;
use crate::FlockElement;
use crate::FlockUnlock;
use crate::ExclusiveFlock;
use crate::SharedFlock;


/// Type for securely creating and securely managing 'flock' locks.
#[derive(Debug)]
pub struct FlockLock<T> where T: FlockElement + WaitFlockUnlock {
	safe_lock_data: SafeUnlockFlock<T>,
}

impl<T> FlockLock<T> where T: FlockElement + WaitFlockUnlock {
	
	/// Create lock surveillance structure, unsafe because it 
	/// is not known if a lock has been created before.
	#[inline]
	pub unsafe fn new(t: T) -> Self {
		Self {
			safe_lock_data: SafeUnlockFlock::new(t)
		}
	}
	
	//safe new
	#[inline(always)]
	pub fn wait_exclusive_lock(f: T) -> Result<FlockLock<T>, FlockError<T>> where T: ExclusiveFlock {
		ExclusiveFlock::wait_lock(f)
	}
	#[inline(always)]
	pub fn try_exclusive_lock(f: T) -> Result<FlockLock<T>, FlockError<T>> where T: ExclusiveFlock {
		ExclusiveFlock::try_lock(f)
	}
	
	
	#[inline(always)]
	pub fn wait_exclusive_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(f: T, function: Fn) -> Result<Fr, FlockFnError<T, Fn, Fr>> where T: ExclusiveFlock {
		ExclusiveFlock::wait_lock_fn(f, function)
	}
	#[inline(always)]
	pub fn try_exclusive_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(f: T, function: Fn) -> Result<Fr, FlockFnError<T, Fn, Fr>> where T: ExclusiveFlock {
		ExclusiveFlock::try_lock_fn(f, function)
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
	pub fn wait_shared_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(f: T, function: Fn) -> Result<Fr, FlockFnError<T, Fn, Fr>> where T: SharedFlock {
		SharedFlock::wait_lock_fn(f, function)
	}
	#[inline(always)]
	pub fn try_shared_lock_fn<Fn: FnOnce(SafeUnlockFlock<T>) -> Fr, Fr>(f: T, function: Fn) -> Result<Fr, FlockFnError<T, Fn, Fr>> where T: SharedFlock {
		SharedFlock::try_lock_fn(f, function)
	}
	//
	
	#[inline]
	pub fn new_block_point(&self) -> &Self {
		&self
	}
	
	#[inline(always)]
	pub fn as_safe_unlock(&self) -> &SafeUnlockFlock<T> {
		&self.safe_lock_data
	}
	
	/// Exclude the current shell and also return the `flock` lock control controller.
	#[inline]
	pub fn to_safe_unlock(self) -> SafeUnlockFlock<T> {
		self.safe_lock_data
	}
	
	/// Destroy the 'flock' lock, return a good result or error.
	#[inline(always)]
	pub fn unlock(self) -> Result<T::UnlockResult, std::io::Error> {
		self.safe_lock_data.unlock()
	}
	
	/// Destroy the 'flock' lock, return a good result or error.
	#[inline(always)]
	pub fn unlock_no_result(self) where T: FlockUnlock {
		self.safe_lock_data.unlock_no_result()
	}
	
	
	/// Destroy the "flock" lock, return data and error data.
	#[cfg(feature = "nightly")]
	#[inline(always)]
	pub fn data_unlock(self) -> (T, Result<T::UnlockResult, std::io::Error>) {
		SafeUnlockFlock::data_unlock(self.safe_lock_data)
	}
	
	/// Destroy the "flock" lock, return data.
	#[cfg(feature = "nightly")]
	#[inline(always)]
	pub fn data_unlock_no_err_result(self) -> T where T: FlockUnlock {
		SafeUnlockFlock::data_unlock_no_err_result(self.safe_lock_data)
	}
}

impl<T> AsRef<T> for FlockLock<T> where T: FlockElement + WaitFlockUnlock {
	#[inline(always)]
	fn as_ref(&self) -> &T {
		&self.safe_lock_data
	}
}
impl<T> AsMut<T> for FlockLock<T> where T: FlockElement + WaitFlockUnlock {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut T {
		&mut self.safe_lock_data
	}
}


impl<T> Deref for FlockLock<T> where T: FlockElement + WaitFlockUnlock {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.safe_lock_data.deref()
	}
}

impl<T> DerefMut for FlockLock<T> where T: FlockElement + WaitFlockUnlock {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.safe_lock_data.deref_mut()
	}
}

impl<T> BehOsRelease for FlockLock<T> where T: FlockElement + WaitFlockUnlock {
	type Ok = Self;
	type Err = FlockError<Self::Data>;
	type Data = T;
	
	#[inline(always)]
	fn ok(t: Self::Data) -> Self::Ok {
		unsafe { Self::Ok::new(t) }
	}
	
	#[inline(always)]
	fn err(t: Self::Data, err: std::io::Error) -> Self::Err {
		Self::Err::new(t, err)
	}
}






