
pub trait FlockUnlock {
	// Try
	//
	/// Destroy the 'flock 'lock without checking for errors, this function is used in Drop.
	unsafe fn try_unlock_no_result(&mut self);
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn try_unlock(&mut self) -> Result<(), std::io::Error>;
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn try_unlock_fn<F: FnOnce() -> R, FE: FnOnce(std::io::Error) -> R, R>(&mut self, next: F, errf: FE) -> R;
	
	// Wait
	//
	/// Destroy the 'flock 'lock without checking for errors, this function is used in Drop.
	unsafe fn wait_unlock_no_result(&mut self);
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn wait_unlock(&mut self) -> Result<(), std::io::Error>;
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn wait_unlock_fn<F: FnOnce() -> R, FE: FnOnce(std::io::Error) -> R, R>(&mut self, next: F, errf: FE) -> R;

	
	// Why initially unsafe?
	// 
	// The structure should be destroyed after calling these functions, 
	// but since Drop `&mut self`; we cannot guarantee this outside our library.
	//
}

impl<T> FlockUnlock for T where T: WaitFlockUnlock + TryFlockUnlock {	
	#[inline(always)]
	unsafe fn try_unlock_no_result(&mut self) {
		TryFlockUnlock::unlock_no_result(self)
	}

	#[inline(always)]
	unsafe fn try_unlock(&mut self) -> Result<(), std::io::Error> {
		TryFlockUnlock::unlock(self)	
	}
	
	#[inline(always)]
	unsafe fn try_unlock_fn<F: FnOnce() -> R, FE: FnOnce(std::io::Error) -> R, R>(&mut self, next: F, errf: FE) -> R {
		TryFlockUnlock::unlock_fn(self, next, errf)
	}
	
	
	#[inline(always)]
	unsafe fn wait_unlock_no_result(&mut self) {
		WaitFlockUnlock::unlock_no_result(self)	
	}
	
	#[inline(always)]
	unsafe fn wait_unlock(&mut self) -> Result<(), std::io::Error> {
		WaitFlockUnlock::unlock(self)	
	}
	
	#[inline(always)]
	unsafe fn wait_unlock_fn<F: FnOnce() -> R, FE: FnOnce(std::io::Error) -> R, R>(&mut self, next: F, errf: FE) -> R {
		WaitFlockUnlock::unlock_fn(self, next, errf)
	}
}


/// Generic describing the function of destroying 'flock' locks.
pub trait TryFlockUnlock {
	/// Destroy the 'flock 'lock without checking for errors, this function is used in Drop.
	unsafe fn unlock_no_result(&mut self);
	
	/// Destroy 'flock' lock, also check errors.
	#[inline(always)]
	unsafe fn unlock(&mut self) -> Result<(), std::io::Error> {
		TryFlockUnlock::unlock_fn(
			self,
			|| Ok(()),
			|e| Err(e)
		)
	}
	
	unsafe fn unlock_fn<F: FnOnce() -> R, FE: FnOnce(std::io::Error) -> R, R>(&mut self, next: F, errf: FE) -> R;
	
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
	#[inline(always)]
	unsafe fn unlock(&mut self) -> Result<(), std::io::Error> {
		WaitFlockUnlock::unlock_fn(
			self,
			|| Ok(()),
			|e| Err(e)
		)
	}
	
	/// Destroy 'flock' lock, also check errors.
	unsafe fn unlock_fn<F: FnOnce() -> R, FE: FnOnce(std::io::Error) -> R, R>(&mut self, next: F, errf: FE) -> R;
	
	// Why initially unsafe?
	// 
	// The structure should be destroyed after calling these functions, 
	// but since Drop `&mut self`; we cannot guarantee this outside our library.
	//
}
