

use crate::RawConstructorElement;
use crate::sys::FlockElement;
use std::ops::DerefMut;
use std::ops::Deref;
use std::io;

pub trait FlockUnlock {	
	fn flock_unlock(&mut self) -> Result<(), io::Error>;
}


#[derive(Debug)]
pub struct UnlockFlock<T> where T: FlockUnlock {
	value: T
}

impl<T> UnlockFlock<T> where T: FlockUnlock {
	#[inline]
	pub fn new(t: T) -> Self {
		Self {
			value: t	
		}
	}
}

impl<T> RawConstructorElement for UnlockFlock<T> where T: FlockElement + FlockUnlock {
	type ConstResult = UnlockFlock<T>;
	type Arg = T;
	
	#[inline(always)]
	fn raw_constructor(t: Self::Arg) -> Self::ConstResult {
		Self::ConstResult::new(t)
	}	
}


impl<T, A, R> RawConstructorElement for (T, A) 
	where 
	T: FlockElement + FlockUnlock,
	A: FnOnce(UnlockFlock<T>) -> R
{
	type ConstResult = R;
	type Arg = (T, A);
	
	#[inline(always)]
	fn raw_constructor((this, f): Self::Arg) -> Self::ConstResult {
		f(From::from(this))
	}	
}



impl<T> Deref for UnlockFlock<T> where T: FlockUnlock {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.value	
	}
}

impl<T> DerefMut for UnlockFlock<T> where T: FlockUnlock {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut T {
		&mut self.value
	}
}

impl<T> From<T> for UnlockFlock<T> where T: FlockUnlock {
	#[inline(always)]
	fn from(t: T) -> Self {
		Self::new(t)	
	}
}

impl<T> Drop for UnlockFlock<T> where T: FlockUnlock {
	fn drop(&mut self) {
		//let _e = crate::sys::unlock(self.0);
		let _e = self.value.flock_unlock();
	}
}
