
use crate::unlock::WaitFlockUnlockRange;
use crate::sys::FlockRangePNum;
use crate::range::FlockRangeFPrimitive;
use crate::range::FlockRangeFull;
use core::ops::DerefMut;
use core::ops::Deref;
use crate::FlockLock;
use crate::unlock::WaitFlockUnlock;
use crate::element::FlockElement;

/// Type for securely creating and securely managing 'flock' locks.
#[derive(/*Copy, */Clone/*, Default*/, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FlockRangeLock<T> where T: FlockElement + WaitFlockUnlock + WaitFlockUnlockRange {
	data: FlockLock<T>,
	range: FlockRangeFPrimitive,
}

impl<T> FlockRangeLock<T> where T: FlockElement + WaitFlockUnlock + WaitFlockUnlockRange {
	/// Form from an already existing flock with the required range.
	#[inline]
	pub unsafe fn from_flock(data: FlockLock<T>, range: impl Into<FlockRangeFPrimitive>) -> Self {
		let range = range.into();
		
		Self {
			data,
			range
		}
	}
	
	/// Is FlockRangeLock a wrapper with values, or is it actually a transparent value with no false data.
	#[inline(always)]
	pub const fn is_repr_transparent(&self) -> bool {
		false
	}
}

impl<T> Default for FlockRangeLock<T> where T: Default + FlockElement + WaitFlockUnlock + WaitFlockUnlockRange {
	#[inline]
	fn default() -> Self {
		let flock = Default::default();
		let range = FlockRangeFull::<FlockRangePNum>::full()
			.into_primitive();
			
		let range = unsafe {
			range.into_ignore_prangechecker()
		};
		
		unsafe {
			Self::from_flock(flock, range)
		}
	}
}

impl<T> Deref for FlockRangeLock<T> where T: FlockElement + WaitFlockUnlock + WaitFlockUnlockRange {
	type Target = FlockLock<T>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<T> DerefMut for FlockRangeLock<T> where T: FlockElement + WaitFlockUnlock + WaitFlockUnlockRange {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}

impl<T> AsRef<T> for FlockRangeLock<T> where T: FlockElement + WaitFlockUnlock + WaitFlockUnlockRange {
	#[inline(always)]
	fn as_ref(&self) -> &T {
		&self.data
	}
}

impl<T> AsMut<T> for FlockRangeLock<T> where T: FlockElement + WaitFlockUnlock + WaitFlockUnlockRange {
	#[inline(always)]
	fn as_mut(&mut self) -> &mut T {
		&mut self.data
	}
}

impl<T> Drop for FlockRangeLock<T> where T: FlockElement + WaitFlockUnlock + WaitFlockUnlockRange {
	fn drop(&mut self) {
		unsafe {
			// Forget unlocking from FlockLock and perform our own unlocking.
			//
			let range = self.range; // copy
			WaitFlockUnlockRange::unlock_range_no_result(self.as_mut_data(), range);
			
			self.data.nomove_ignore_unlock_no_result();
		}
	}
}
