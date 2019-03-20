
use crate::ExclusiveFlockFn;
use crate::unlock::UnlockFlock;
use crate::sys::FlockElement;
use crate::SharedFlockFn;
use crate::ExclusiveFlock;
use crate::SharedFlock;
use crate::FlockLock;
use crate::FlockError;

///Constructor, generalized for 'Flock'
pub trait ToFlock {
	#[inline(always)]
	fn wait_exclusive_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: ExclusiveFlock + Sized {
		ExclusiveFlock::wait_lock(self)
	}

	
	#[inline(always)]
	fn wait_exclusive_lock_fn<A: FnOnce(UnlockFlock<Self>) -> R, R>(self, f: A) -> Result<R, FlockError<(Self, A)>> where Self: ExclusiveFlockFn + Sized {
		ExclusiveFlockFn::wait_lock_fn(self, f)
	}
	

	#[inline(always)]
	fn try_exclusive_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: ExclusiveFlock + Sized {
		ExclusiveFlock::try_lock(self)
	}

	
	#[inline(always)]
	fn try_exclusive_lock_fn<A: FnOnce(UnlockFlock<Self>) -> R, R>(self, f: A) -> Result<R, FlockError<(Self, A)>> where Self: ExclusiveFlockFn + Sized {
		ExclusiveFlockFn::try_lock_fn(self, f)
	}
	
	#[inline(always)]
	fn wait_shared_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: SharedFlock + Sized {
		SharedFlock::wait_lock(self)
	}
	
	#[inline(always)]
	fn wait_shared_lock_fn<A: FnOnce(UnlockFlock<Self>) -> R, R>(self, f: A) -> Result<R, FlockError<(Self, A)>> where Self: SharedFlockFn + Sized {
		SharedFlockFn::wait_lock_fn(self, f)
	}

	#[inline(always)]
	fn try_shared_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: SharedFlock + Sized {
		SharedFlock::try_lock(self)
	}

	#[inline(always)]
	fn try_shared_lock_fn<A: FnOnce(UnlockFlock<Self>) -> R, R>(self, f: A) -> Result<R, FlockError<(Self, A)>> where Self: SharedFlockFn + Sized {
		SharedFlockFn::try_lock_fn(self, f)
	}
}

impl<F> ToFlock for F where F: FlockElement {}