
use core::fmt::Debug;

#[cfg(windows)]
pub use crate::sys::RangeStartFlock;
#[cfg(windows)]
pub use crate::sys::RangeEndFlock;

pub trait RangeFlock: Sized + Debug {
	type NumType: Copy;
	type FinalTransform: RangeFlock<
		NumType = Self::NumType
	>;
	
	fn is_valid(&self) -> bool;	
	fn final_transform(self) -> Self::FinalTransform;
	fn get_range<R>(self, next: impl FnOnce(Self::NumType, Self::NumType) -> R) -> R;
}

impl RangeFlock for () { // NULL
	type NumType = usize;
	type FinalTransform = Self;
	
	#[inline(always)]
	fn is_valid(&self) -> bool {
		true
	}
	
	#[inline(always)]
	fn final_transform(self) -> Self::FinalTransform {
		self
	}
	
	#[inline(always)]
	fn get_range<R>(self, _next: impl FnOnce(Self::NumType, Self::NumType) -> R) -> R {
		unimplemented!("This code should never be called and is required as a stub.");
	}
}
