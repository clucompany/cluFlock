
use std::ops::DerefMut;
use std::ops::Deref;
use crate::RawConstructorElement;
use crate::FlockError;
use crate::FlockElement;
use crate::FlockUnlock;
use crate::ExclusiveFlock;
use crate::SharedFlock;

#[derive(Debug)]
pub struct FlockLock<T> where T: FlockElement + FlockUnlock {
	element: T,
}


impl<T> Deref for FlockLock<T> where T: FlockElement + FlockUnlock {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.element	
	}
}

impl<T> DerefMut for FlockLock<T> where T: FlockElement + FlockUnlock {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.element	
	}
}


impl<T> RawConstructorElement for FlockLock<T> where T: FlockElement + FlockUnlock {
	type ConstResult = FlockLock<T>;
	type Arg = T;
	
	fn raw_constructor(t: Self::Arg) -> Self::ConstResult {
		Self::ConstResult {
			element: t,
		}
	}	
}

impl<T: ExclusiveFlock> FlockLock<T> where T: FlockElement + FlockUnlock {
	#[inline(always)]
	pub fn wait_exclusive_lock(f: T) -> Result<FlockLock<T>, FlockError<T>> {
		ExclusiveFlock::wait_lock(f)
	}
	#[inline(always)]
	pub fn try_exclusive_lock(f: T) -> Result<FlockLock<T>, FlockError<T>> {
		ExclusiveFlock::try_lock(f)
	}	
}

impl<T: SharedFlock> FlockLock<T> where T: FlockElement + FlockUnlock {
	#[inline(always)]
	pub fn wait_shared_lock(f: T) -> Result<FlockLock<T>, FlockError<T>> {
		SharedFlock::wait_lock(f)
	}
	
	#[inline(always)]
	pub fn try_shared_lock(f: T) -> Result<FlockLock<T>, FlockError<T>> {
		SharedFlock::try_lock(f)
	}
}


impl<T> Drop for FlockLock<T> where T: FlockElement + FlockUnlock {
	fn drop(&mut self) {
		let _e = self.element.flock_unlock();
	}
}
