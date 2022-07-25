
//! Sets of flags that define range checking behavior.

use core::fmt::Debug;

/// Specifies the behavior of the range check.
pub trait FlockRangePNumBehChecker: 'static + Sized + Debug + Clone + Copy + PartialEq + PartialOrd + Eq + Ord {
	/// Specifies the behavior of the range check.
	const IS_EN_RANGECHECK: bool;
}

/// The range check will never be performed.
/// Useful for full ranges created with the FlockRange structure, 
/// or for unsafe code that requires high performance.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum FlockRangeBehCheckIgnore {}
impl FlockRangePNumBehChecker for FlockRangeBehCheckIgnore {
	/// Specifies the behavior of the range check.
	const IS_EN_RANGECHECK: bool = false;
}

/// There will always be a range check.
/// Default and safe for ranges computed at run time.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum FlockRangeBehCheckAlways {}
impl FlockRangePNumBehChecker for FlockRangeBehCheckAlways {
	/// Specifies the behavior of the range check.
	const IS_EN_RANGECHECK: bool = true;
}
