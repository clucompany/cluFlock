
use crate::ExclusiveFlock;
use crate::SharedFlock;
use crate::FlockLock;
use crate::err::FlockError;
use crate::element::FlockElement;

/// Convenient conversion of previously used values ​​to cluFlock.
pub trait ToFlock {
	fn wait_exclusive_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: ExclusiveFlock;
	fn wait_exclusive_lock_fn<F: FnOnce(FlockLock<Self>) -> R, FE: FnOnce(FlockError<Self>) -> R, R>(self, next: F, errf: FE) -> R where Self: ExclusiveFlock;

	fn try_exclusive_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: ExclusiveFlock;
	fn try_exclusive_lock_fn<F: FnOnce(FlockLock<Self>) -> R, FE: FnOnce(FlockError<Self>) -> R, R>(self, next: F, errf: FE) -> R where Self: ExclusiveFlock;
	
	
	fn wait_shared_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: SharedFlock;
	fn wait_shared_lock_fn<F: FnOnce(FlockLock<Self>) -> R, FE: FnOnce(FlockError<Self>) -> R, R>(self, next: F, errf: FE) -> R where Self: SharedFlock;

	fn try_shared_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: SharedFlock;
	fn try_shared_lock_fn<F: FnOnce(FlockLock<Self>) -> R, FE: FnOnce(FlockError<Self>) -> R, R>(self, next: F, errf: FE) -> R where Self: SharedFlock;
}

impl<T> ToFlock for T where T: FlockElement {
	#[inline(always)]
	fn wait_exclusive_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: ExclusiveFlock {
		ExclusiveFlock::wait_lock(self)
	}
	
	#[inline(always)]
	fn wait_exclusive_lock_fn<F: FnOnce(FlockLock<Self>) -> R, FE: FnOnce(FlockError<Self>) -> R, R>(self, next: F, errf: FE) -> R where Self: ExclusiveFlock {
		ExclusiveFlock::wait_lock_fn(self, next, errf)
	}
	

	#[inline(always)]
	fn try_exclusive_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: ExclusiveFlock {
		ExclusiveFlock::try_lock(self)
	}

	
	#[inline(always)]
	fn try_exclusive_lock_fn<F: FnOnce(FlockLock<Self>) -> R, FE: FnOnce(FlockError<Self>) -> R, R>(self, next: F, errf: FE) -> R where Self: ExclusiveFlock {
		ExclusiveFlock::try_lock_fn(self, next, errf)
	}
	
	#[inline(always)]
	fn wait_shared_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: SharedFlock {
		SharedFlock::wait_lock(self)
	}
	
	#[inline(always)]
	fn wait_shared_lock_fn<F: FnOnce(FlockLock<Self>) -> R, FE: FnOnce(FlockError<Self>) -> R, R>(self, next: F, errf: FE) -> R where Self: SharedFlock {
		SharedFlock::wait_lock_fn(self, next, errf)
	}

	#[inline(always)]
	fn try_shared_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: SharedFlock {
		SharedFlock::try_lock(self)
	}

	#[inline(always)]
	fn try_shared_lock_fn<F: FnOnce(FlockLock<Self>) -> R, FE: FnOnce(FlockError<Self>) -> R, R>(self, next: F, errf: FE) -> R where Self: SharedFlock {
		SharedFlock::try_lock_fn(self, next, errf)
	}
	
}