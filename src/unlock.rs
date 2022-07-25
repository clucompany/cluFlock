
//! Traits needed to unlock existing locks.

use crate::range::checker::FlockRangePNumBehChecker;
use crate::range::pnum::FlockRangePNumBeh;
use crate::range::FlockRange;
use crate::err::IoError;

/// Trait defining support for all possible unlocks (WaitFlockUnlock + TryFlockUnlock).
pub trait FlockUnlock {
	// Try
	//
	/// Destroy the 'flock 'lock without checking for errors, 
	/// this function is used in Drop.
	unsafe fn try_unlock_no_result(&mut self);
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn try_unlock(&mut self) -> Result<(), IoError>;
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn try_unlock_fn<R>(&mut self, next: impl FnOnce() -> R, errf: impl FnOnce(IoError) -> R) -> R;
	
	// Wait
	//
	/// Destroy the 'flock 'lock without checking for errors, 
	/// this function is used in Drop.
	unsafe fn wait_unlock_no_result(&mut self);
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn wait_unlock(&mut self) -> Result<(), IoError>;
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn wait_unlock_fn<R>(&mut self, next: impl FnOnce() -> R, errf: impl FnOnce(IoError) -> R) -> R;

	
	// Why initially unsafe?
	// 
	// The structure should be destroyed after calling these functions, 
	// but since Drop `&mut self`; we cannot guarantee this outside our library.
	//
}

impl<T> FlockUnlock for T where T: WaitFlockUnlock + TryFlockUnlock {
	/// Destroy the 'flock 'lock without checking for errors, 
	/// this function is used in Drop.
	#[inline(always)]
	unsafe fn try_unlock_no_result(&mut self) {
		TryFlockUnlock::unlock_no_result(self)
	}

	/// Destroy 'flock' lock, also check errors.
	#[inline(always)]
	unsafe fn try_unlock(&mut self) -> Result<(), IoError> {
		TryFlockUnlock::unlock(self)
	}
	
	/// Destroy 'flock' lock, also check errors.
	#[inline(always)]
	unsafe fn try_unlock_fn<R>(&mut self, next: impl FnOnce() -> R, errf: impl FnOnce(IoError) -> R) -> R {
		TryFlockUnlock::unlock_fn(self, next, errf)
	}
	
	/// Destroy the 'flock 'lock without checking for errors, 
	/// this function is used in Drop.
	#[inline(always)]
	unsafe fn wait_unlock_no_result(&mut self) {
		WaitFlockUnlock::unlock_no_result(self)
	}
	
	/// Destroy 'flock' lock, also check errors.
	#[inline(always)]
	unsafe fn wait_unlock(&mut self) -> Result<(), IoError> {
		WaitFlockUnlock::unlock(self)	
	}
	
	/// Destroy 'flock' lock, also check errors.
	#[inline(always)]
	unsafe fn wait_unlock_fn<R>(&mut self, next: impl FnOnce() -> R, errf: impl FnOnce(IoError) -> R) -> R {
		WaitFlockUnlock::unlock_fn(self, next, errf)
	}
}


/// Generic describing the function of destroying 'flock' locks.
pub trait TryFlockUnlock {
	/// Destroy the 'flock 'lock without checking for errors, this function is used in Drop.
	unsafe fn unlock_no_result(&mut self);
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn unlock(&mut self) -> Result<(), IoError>;
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn unlock_fn<R>(&mut self, next: impl FnOnce() -> R, errf: impl FnOnce(IoError) -> R) -> R;
	
	// Why initially unsafe?
	// 
	// The structure should be destroyed after calling these functions, 
	// but since Drop `&mut self`; we cannot guarantee this outside our library.
	//
}

/// Generic describing the function of destroying 'flock' locks.
pub trait WaitFlockUnlock {
	/// Destroy the 'flock 'lock without checking for errors, this function is used in Drop.
	unsafe fn unlock_no_result(&mut self);
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn unlock(&mut self) -> Result<(), IoError>;
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn unlock_fn<R>(&mut self, next: impl FnOnce() -> R, errf: impl FnOnce(IoError) -> R) -> R;
	
	// Why initially unsafe?
	// 
	// The structure should be destroyed after calling these functions, 
	// but since Drop `&mut self`; we cannot guarantee this outside our library.
	//
}

/// Generic describing the function of destroying 'flock' locks.
pub trait WaitFlockUnlockRange {
	/// Destroy the 'flock 'lock without checking for errors, this function is used in Drop.
	unsafe fn unlock_range_no_result<NS, NE, C>(&mut self, range: impl Into<FlockRange<NS, NE, C>>) where NS: FlockRangePNumBeh, NE: FlockRangePNumBeh + PartialOrd<NS>, C: FlockRangePNumBehChecker;
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn unlock_range<NS, NE, C>(&mut self, range: impl Into<FlockRange<NS, NE, C>>) -> Result<(), IoError> where NS: FlockRangePNumBeh, NE: FlockRangePNumBeh + PartialOrd<NS>, C: FlockRangePNumBehChecker;
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn unlock_range_fn<R, NS, NE, C>(&mut self, range: impl Into<FlockRange<NS, NE, C>>, next: impl FnOnce() -> R, errf: impl FnOnce(IoError) -> R) -> R where NS: FlockRangePNumBeh, NE: FlockRangePNumBeh + PartialOrd<NS>, C: FlockRangePNumBehChecker;
	
	// Why initially unsafe?
	// 
	// The structure should be destroyed after calling these functions, 
	// but since Drop `&mut self`; we cannot guarantee this outside our library.
	//
}
