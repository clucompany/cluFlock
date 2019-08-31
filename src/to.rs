
use crate::err::FlockFnError;
use crate::SafeUnlockFlock;
use crate::sys::FlockElement;
use crate::ExclusiveFlock;
use crate::SharedFlock;
use crate::FlockLock;
use crate::err::FlockError;

///Constructor, generalized for 'Flock'
pub trait ToFlock where Self: Sized {
	fn wait_exclusive_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: ExclusiveFlock;
	fn wait_exclusive_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> where Self: ExclusiveFlock;
	

	fn try_exclusive_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: ExclusiveFlock;
	fn try_exclusive_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> where Self: ExclusiveFlock;
	
	fn wait_shared_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: SharedFlock;
	fn wait_shared_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> where Self: SharedFlock;

	fn try_shared_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: SharedFlock;
	fn try_shared_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> where Self: SharedFlock;
}

impl<T> ToFlock for T where T: FlockElement + Sized {
	#[inline(always)]
	fn wait_exclusive_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: ExclusiveFlock {
		ExclusiveFlock::wait_lock(self)
	}

	
	#[inline(always)]
	fn wait_exclusive_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> where Self: ExclusiveFlock {
		ExclusiveFlock::wait_lock_fn(self, f)
	}
	

	#[inline(always)]
	fn try_exclusive_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: ExclusiveFlock {
		ExclusiveFlock::try_lock(self)
	}

	
	#[inline(always)]
	fn try_exclusive_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> where Self: ExclusiveFlock {
		ExclusiveFlock::try_lock_fn(self, f)
	}
	
	#[inline(always)]
	fn wait_shared_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: SharedFlock {
		SharedFlock::wait_lock(self)
	}
	
	#[inline(always)]
	fn wait_shared_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> where Self: SharedFlock {
		SharedFlock::wait_lock_fn(self, f)
	}

	#[inline(always)]
	fn try_shared_lock(self) -> Result<FlockLock<Self>, FlockError<Self>> where Self: SharedFlock {
		SharedFlock::try_lock(self)
	}

	#[inline(always)]
	fn try_shared_lock_fn<Fn: FnOnce(SafeUnlockFlock<Self>) -> Fr, Fr>(self, f: Fn) -> Result<Fr, FlockFnError<Self, Fn, Fr>> where Self: SharedFlock {
		SharedFlock::try_lock_fn(self, f)
	}
	
}