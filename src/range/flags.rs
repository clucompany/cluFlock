
//! Sets of flags that make up a range and control its behavior.

use core::marker::PhantomData;
use core::cmp::Ordering;
use crate::range::FlockRangePNum;
use crate::range::FlockRangePNumBeh;

/// An empty numeric attribute (only needed to describe the range function).
/// scheme: 
/// false ->	any
/// true ->	null
#[derive(Debug, Clone, Copy, Eq, Ord, Hash)]
pub struct IgnoreNum;

impl FlockRangePNumBeh for IgnoreNum {
	const MIN: IgnoreNum = IgnoreNum;
	const MAX: IgnoreNum = IgnoreNum;
	
	const IS_IGNORE_NUM: bool = true;
	
	#[inline(always)]
	fn get_pnum(self) -> FlockRangePNum {
		panic!("Undefined behavior detected.");
	}
}

impl<N> PartialEq<N> for IgnoreNum where N: FlockRangePNumBeh {
	#[inline(always)]
	fn eq(&self, _b: &N) -> bool {
		panic!("Undefined behavior detected.");
	}
}

impl<N> PartialOrd<N> for IgnoreNum where N: FlockRangePNumBeh {
	#[inline(always)]
	fn partial_cmp(&self, _b: &N) -> Option<Ordering> {
		panic!("Undefined behavior detected.");
	}
}

/// There is only a number that defines the position in the range.
/// scheme:
/// false ->	(A..=B)
/// true ->	(A..=A)
#[derive(Debug, Clone, Copy, Eq, Ord, Hash)]
pub struct OnePosNum;

impl FlockRangePNumBeh for OnePosNum {
	const MIN: OnePosNum = OnePosNum;
	const MAX: OnePosNum = OnePosNum;
	
	const IS_ONEPOS: bool = true;
	
	#[inline(always)]
	fn get_pnum(self) -> FlockRangePNum {
		panic!("Undefined behavior detected.");
	}
}

impl<N> PartialEq<N> for OnePosNum where N: FlockRangePNumBeh {
	#[inline(always)]
	fn eq(&self, _b: &N) -> bool {
		panic!("Undefined behavior detected.");
	}
}

impl<N> PartialOrd<N> for OnePosNum where N: FlockRangePNumBeh {
	#[inline(always)]
	fn partial_cmp(&self, _b: &N) -> Option<Ordering> {
		panic!("Undefined behavior detected.");
	}
}

/// The full range, keeping only the numeric type to make up the final range.
/// scheme:
/// false ->	any
/// true ->	(min..max)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct FullRangeNum<N>(PhantomData<N>) where N: FlockRangePNumBeh;

impl<N> FullRangeNum<N> where N: FlockRangePNumBeh {
	#[inline(always)]
	pub const fn new() -> Self {
		FullRangeNum(PhantomData)
	}
}

impl<N> FlockRangePNumBeh for FullRangeNum<N> where N: FlockRangePNumBeh {
	const MIN: FullRangeNum<N> = FullRangeNum(PhantomData);
	const MAX: FullRangeNum<N> = FullRangeNum(PhantomData);
	
	const IS_FULLRANGE: bool = true;
	
	#[inline(always)]
	fn get_pnum(self) -> FlockRangePNum {
		panic!("Undefined behavior detected.");
	}
	
	#[inline(always)]
	fn get_pnum_min() -> FlockRangePNum {
		N::get_pnum_min()
	}
	
	#[inline(always)]
	fn get_pnum_max() -> FlockRangePNum {
		N::get_pnum_max()
	}
}

impl<N> PartialEq<N> for FullRangeNum<N> where N: FlockRangePNumBeh {
	#[inline(always)]
	fn eq(&self, _b: &N) -> bool {
		panic!("Undefined behavior detected.");
	}
}

impl<N> PartialOrd<N> for FullRangeNum<N> where N: FlockRangePNumBeh {
	#[inline(always)]
	fn partial_cmp(&self, _b: &N) -> Option<Ordering> {
		panic!("Undefined behavior detected.");
	}
}

