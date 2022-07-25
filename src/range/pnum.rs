
//! Determination of the logic of the range operation.

use core::hash::Hash;
use crate::sys::FlockRangePNum;
use core::fmt::Debug;

/// Determination of the logic of the range operation.
pub trait FlockRangePNumBeh: Sized + Debug + Clone + Copy + PartialEq + PartialOrd + Eq + Ord + Hash {
	/// The smallest possible range.
	const MIN: Self;
	/// The maximum possible range.
	const MAX: Self;
	
	/// An empty numeric attribute (only needed to describe the range function).
	/// scheme: 
	/// false ->	any
	/// true ->	null
	const IS_IGNORE_NUM: bool = false;
	/// There is only a number that defines the position in the range.
	/// scheme:
	/// false ->	(A..=B)
	/// true ->	(A..=A)
	const IS_ONEPOS: bool = false;
	
	/// The full range, keeping only the numeric type to make up the final range.
	/// scheme:
	/// false ->	any
	/// true ->	(min..max)
	const IS_FULLRANGE: bool = false;
	
	/// Checking the range for entry into the minimum range
	/// scheme: self >= &Self::MIN
	#[inline(always)]
	fn is_valid_minrange(&self) -> bool {
		if Self::IS_IGNORE_NUM || Self::IS_ONEPOS || Self::IS_FULLRANGE {
			panic!("Undefined behavior detected.");
		}
		
		self >= &Self::MIN
	}
	
	/// Checking the range for entry into the maximum range
	/// scheme: self <= &Self::MAX
	#[inline(always)]
	fn is_valid_maxrange(&self) -> bool {
		if Self::IS_IGNORE_NUM || Self::IS_ONEPOS || Self::IS_FULLRANGE {
			panic!("Undefined behavior detected.");
		}
		
		self <= &Self::MAX
	}
	
	/// Check the full range to make sure it is within the safe range.
	/// scheme: self >= &Self::MIN && self <= &Self::MAX
	#[inline(always)]
	fn is_valid(&self) -> bool {
		if Self::IS_IGNORE_NUM || Self::IS_ONEPOS || Self::IS_FULLRANGE {
			panic!("Undefined behavior detected.");
		}
		
		self.is_valid_minrange() && self.is_valid_maxrange()
	}
	
	/// Check the full range to make sure it is within the maximum of the other range.
	/// scheme: end >= *self
	#[inline(always)]
	fn is_valid_endrange<NE: FlockRangePNumBeh>(&self, end: NE) -> bool where NE: PartialOrd<Self> {
		if Self::IS_IGNORE_NUM || Self::IS_ONEPOS || Self::IS_FULLRANGE {
			panic!("Undefined behavior detected.");
		}
		if NE::IS_IGNORE_NUM || NE::IS_ONEPOS || NE::IS_FULLRANGE {
			panic!("Undefined behavior detected.");
		}
		
		// self <- start
		end >= *self
	}
	
	/// Get part of a range as a number.
	fn get_pnum(self) -> FlockRangePNum;
	
	/// Get the minimum of the range as a number.
	#[inline(always)]
	fn get_pnum_min() -> FlockRangePNum {
		Self::MIN.get_pnum()
	}
	
	/// Get the maximum of the range as a number.
	#[inline(always)]
	fn get_pnum_max() -> FlockRangePNum {
		Self::MAX.get_pnum()
	}
}

#[doc(hidden)]
#[macro_export(crate)]
macro_rules! __make_auto_pnum_type {
	[$($t: ty),*] => {
		$(
			impl PartialEq<$crate::range::flags::OnePosNum> for $t {
				#[inline(always)]
				fn eq(&self, _b: &$crate::range::flags::OnePosNum) -> bool {
					panic!("Undefined behavior detected.");
				}
			}
			
			impl PartialOrd<$crate::range::flags::OnePosNum> for $t {
				#[inline(always)]
				fn partial_cmp(&self, _b: &$crate::range::flags::OnePosNum) -> Option<core::cmp::Ordering> {
					panic!("Undefined behavior detected.");
				}
			}
			
			impl PartialEq<$crate::range::flags::IgnoreNum> for $t {
				#[inline(always)]
				fn eq(&self, _b: &$crate::range::flags::IgnoreNum) -> bool {
					panic!("Undefined behavior detected.");
				}
			}
			
			impl PartialOrd<$crate::range::flags::IgnoreNum> for $t {
				#[inline(always)]
				fn partial_cmp(&self, _b: &$crate::range::flags::IgnoreNum) -> Option<core::cmp::Ordering> {
					panic!("Undefined behavior detected.");
				}
			}
			
			impl<N> PartialEq<$crate::range::flags::FullRangeNum<N>> for $t where N: FlockRangePNumBeh {
				#[inline(always)]
				fn eq(&self, _b: &$crate::range::flags::FullRangeNum<N>) -> bool {
					panic!("Undefined behavior detected.");
				}
			}
			
			impl<N> PartialOrd<$crate::range::flags::FullRangeNum<N>> for $t where N: FlockRangePNumBeh {
				#[inline(always)]
				fn partial_cmp(&self, _b: &$crate::range::flags::FullRangeNum<N>) -> Option<core::cmp::Ordering> {
					panic!("Undefined behavior detected.");
				}
			}
			
		)*
	};
}

#[test]
#[cfg(test)]
fn test_flock_rangepnum_valid() {
	#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
	struct __TestNum(i8);
	impl FlockRangePNumBeh for __TestNum {
		const MIN: __TestNum = __TestNum(0);
		const MAX: __TestNum = __TestNum(2);
		
		#[inline(always)]
		fn get_pnum(self) -> FlockRangePNum {
			self.0 as _
		}
	}
	crate::__make_auto_pnum_type!(__TestNum);
	
	assert_eq!(__TestNum(-1).is_valid(), false);
	assert_eq!(__TestNum(0).is_valid(), true);
	assert_eq!(__TestNum(1).is_valid(), true);
	assert_eq!(__TestNum(2).is_valid(), true);
	assert_eq!(__TestNum(3).is_valid(), false);
}

