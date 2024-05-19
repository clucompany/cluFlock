//! The range used to lock by position in the file (start and end).

use crate::err::IoError;
use crate::err::IoErrorKind;
use crate::range::checker::FlockRangeBehCheckAlways;
use crate::range::checker::FlockRangeBehCheckIgnore;
use crate::range::checker::FlockRangePNumBehChecker;
use crate::range::flags::FullRangeNum;
use crate::range::flags::IgnoreNum;
use crate::range::flags::OnePosNum;
use crate::range::pnum::FlockRangePNumBeh;
use crate::sys::FlockRangePNum;
use core::cmp::Ordering;
use core::fmt::Debug;
use core::hash::Hash;
use core::marker::PhantomData;
use core::ops::Range;
use core::ops::RangeFrom;
use core::ops::RangeFull;
use core::ops::RangeTo;

crate::cfg_std! {
	if #std {} else {
		extern crate alloc;
		use alloc::format;
	}
}

pub mod checker;
pub mod flags;
pub mod pnum;

/// Range defining the start and end positions of the flock.
/// scheme:
/// (A..=B) || (MIN..=MAX) || (..=B) || (A..) || (..).
#[derive(Clone, Copy, Eq, Ord)]
pub struct FlockRange<NS, NE, C>
where
	NS: FlockRangePNumBeh,
	NE: FlockRangePNumBeh,
	C: FlockRangePNumBehChecker,
{
	start: NS,
	end: NE,

	_pp: PhantomData<C>,
}

/// A range covering the entire supported range.
/// scheme: (min..=max)
pub type FlockRangeFull<N> = FlockRangeExpForce<FullRangeNum<N>, IgnoreNum>;
/// A range consisting of one position.
/// scheme: 10
pub type FlockRangeOnePos<N> = FlockRangeExp<OnePosNum, N>;
/// An arbitrary range derived from a rust core.
/// scheme: (10..=12) || (10, 12)
pub type FlockRangeCore<N> = FlockRangeExp<N, N>;
/// The range specifies only the starting position.
/// The final position will be taken as the maximum.
/// scheme: (10..=)
pub type FlockRangeStartPos<N> = FlockRangeExp<N, IgnoreNum>;
/// The range defines only the end position.
/// The starting position will be taken as the minimum.
/// scheme: (..=23)
pub type FlockRangeEndPos<N> = FlockRangeExp<IgnoreNum, N>;

/// Range defining the start and end positions of the flock.
/// Range is ALWAYS checked.
/// scheme: (A..=B) || (MIN..=MAX) || (..=B) || (A..) || (..).
pub type FlockRangeExp<NS, NE> = FlockRange<NS, NE, FlockRangeBehCheckAlways>;
/// Range defining the start and end positions of the flock.
/// Range is NEVER checked.
/// scheme: (A..=B) || (MIN..=MAX) || (..=B) || (A..) || (..).
pub type FlockRangeExpForce<NS, NE> = FlockRange<NS, NE, FlockRangeBehCheckIgnore>;

/// The most simplified and unoptimized type. It is advisable to use in case of duplication of RangeFlock checks.
pub type FlockRangePrimitive = FlockRangeCore<FlockRangePNum>;

/// The most simplified and unoptimized type. It is advisable to use in case of duplication of RangeFlock checks. No range checks.
pub type FlockRangeFPrimitive = FlockRangeExpForce<FlockRangePNum, FlockRangePNum>;

impl<NS, NE, C> Debug for FlockRange<NS, NE, C>
where
	NS: FlockRangePNumBeh,
	NE: FlockRangePNumBeh,
	C: FlockRangePNumBehChecker,
{
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
		let core_range = (*self).into_core();

		Debug::fmt(&core_range, f)
	}
}

impl<NS, NE, C> Hash for FlockRange<NS, NE, C>
where
	NS: FlockRangePNumBeh,
	NE: FlockRangePNumBeh,
	C: FlockRangePNumBehChecker,
{
	#[inline]
	fn hash<H>(&self, hasher: &mut H)
	where
		H: core::hash::Hasher,
	{
		let (start, end) = self.get();

		Hash::hash(&start, hasher);
		Hash::hash(&end, hasher);
	}
}

impl<N> Default for FlockRangeFull<N>
where
	N: FlockRangePNumBeh,
{
	#[inline(always)]
	fn default() -> Self {
		FlockRangeFull::<N>::full()
	}
}

/// Error describing errors when checking range in FlockRange.
/// (!! Note that InvalidStartEndPos, InvalidStartEndPos2 and InvalidStartEndPos3 define the same error type but have different internal data types).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlockRangeErr<NS, NE>
where
	NS: FlockRangePNumBeh,
	NE: FlockRangePNumBeh,
{
	/// scheme: X..max
	InvalidStartPos(NS),
	/// scheme: min..X
	InvalidEndPos(NE),

	/// InvalidStartEndPos: scheme: X..X2
	InvalidStartEndPos(NS, NE),
	/// InvalidStartEndPos: scheme: X..X2
	InvalidStartEndPos2(NE, NE),
	/// InvalidStartEndPos: scheme: X..X2
	InvalidStartEndPos3(NS, NS),

	/// scheme: pos X
	InvalidShortRange(NE),
}

impl<NS, NE> FlockRangeErr<NS, NE>
where
	NS: FlockRangePNumBeh,
	NE: FlockRangePNumBeh,
{
	/// Make a complete IoError from FlockRangeErr.
	pub fn make_io_error(self) -> IoError {
		let info = match self {
			Self::InvalidStartPos(a) => format!(
				"FlockRangeErr::InvalidStartPos, {:?}..max, unsupported range.",
				a
			),
			Self::InvalidEndPos(a) => format!(
				"FlockRangeErr::InvalidEndPos, min..{:?}, unsupported range.",
				a
			),

			Self::InvalidStartEndPos(a, b) => format!(
				"FlockRangeErr::InvalidStartEndPos, {:?}..{:?}, unsupported range.",
				a, b
			),
			Self::InvalidStartEndPos2(a, b) => format!(
				"FlockRangeErr::InvalidStartEndPos, {:?}..{:?}, unsupported range.",
				a, b
			),
			Self::InvalidStartEndPos3(a, b) => format!(
				"FlockRangeErr::InvalidStartEndPos, {:?}..{:?}, unsupported range.",
				a, b
			),

			Self::InvalidShortRange(a) => format!(
				"FlockRangeErr::InvalidShortRange, pos: {:?}, unsupported range.",
				a
			),
		};

		IoError::new(IoErrorKind::InvalidInput, info)
	}
}

impl FlockRangePrimitive {
	/*
	/// Creation of a range with the most primitive NS, NE in order to simplify the use.
	#[inline(always)]
	pub const fn from_primitive(start: FlockRangePNum, end: FlockRangePNum) -> FlockRangePrimitive {
		const_new(start, end)
	}
	*/

	/// Disable range checking, any number in the range will not be checked.
	#[inline(always)]
	pub const unsafe fn into_ignore_prangechecker(self) -> FlockRangeFPrimitive {
		FlockRange {
			start: self.start,
			end: self.end,

			_pp: PhantomData,
		}
	}
}

impl<NS, NE> FlockRangeExp<NS, NE>
where
	NS: FlockRangePNumBeh,
	NE: FlockRangePNumBeh,
{
	/// Disable range checking, any number in the range will not be checked.
	#[inline(always)]
	pub const unsafe fn into_ignore_rangechecker(self) -> FlockRangeExpForce<NS, NE> {
		FlockRange {
			start: self.start,
			end: self.end,

			_pp: PhantomData,
		}
	}

	/// Check the range and cast it to a range type with no checks to eliminate further checks.
	#[inline]
	pub fn into_check_range(
		self,
	) -> Result<FlockRangeExpForce<NS, NE>, (FlockRangeExp<NS, NE>, FlockRangeErr<NS, NE>)>
	where
		NE: PartialOrd<NS>,
	{
		self.check_range_fn(
			|| {
				let sself = unsafe { self.into_ignore_rangechecker() };
				Ok(sself)
			},
			|e| Err((self, e)),
		)
	}
}

impl<NS, NE> From<(NS, NE)> for FlockRangeExp<NS, NE>
where
	NS: FlockRangePNumBeh,
	NE: FlockRangePNumBeh,
{
	#[inline(always)]
	fn from((start, end): (NS, NE)) -> Self {
		FlockRange::new(start, end)
	}
}

impl<N> From<Range<N>> for FlockRangeExp<N, N>
where
	N: FlockRangePNumBeh,
{
	#[inline(always)]
	fn from(range: Range<N>) -> Self {
		FlockRange::from_core(range)
	}
}

impl<N> From<RangeTo<N>> for FlockRangeEndPos<N>
where
	N: FlockRangePNumBeh,
{
	#[inline(always)]
	fn from(range: RangeTo<N>) -> Self {
		FlockRange::from_endpos(range.end)
	}
}

impl<N> From<RangeFrom<N>> for FlockRangeStartPos<N>
where
	N: FlockRangePNumBeh,
{
	#[inline(always)]
	fn from(range: RangeFrom<N>) -> Self {
		FlockRange::from_startpos(range.start)
	}
}

impl<N> From<RangeFull> for FlockRangeFull<N>
where
	N: FlockRangePNumBeh,
{
	#[inline(always)]
	fn from(_range: RangeFull) -> Self {
		FlockRangeFull::<N>::full()
	}
}

impl<NS> From<NS> for FlockRangeOnePos<NS>
where
	NS: FlockRangePNumBeh,
{
	#[inline(always)]
	fn from(one_pos: NS) -> Self {
		FlockRange::new_one_position(one_pos)
	}
}

#[inline]
const fn const_new<NS, NE>(start: NS, end: NE) -> FlockRangeExp<NS, NE>
where
	NS: FlockRangePNumBeh,
	NE: FlockRangePNumBeh,
{
	FlockRange {
		start,
		end,

		_pp: PhantomData,
	}
}

impl<N> FlockRangeFull<N>
where
	N: FlockRangePNumBeh,
{
	/// Create a complete range (minimum..maximum).
	/// Range numbers will be set automatically.
	/// The range will not be checked for safety.
	#[inline(always)]
	pub const fn full() -> Self {
		let range = const_new(FullRangeNum::new(), IgnoreNum);

		unsafe {
			range.into_ignore_rangechecker() // safe
		}
	}
}

impl<N> FlockRangeOnePos<N>
where
	N: FlockRangePNumBeh,
{
	///
	#[inline]
	pub const fn new_one_position(pos: N) -> Self {
		const_new(OnePosNum, pos)
	}
}

impl<N> FlockRangeStartPos<N>
where
	N: FlockRangePNumBeh,
{
	/// Create a range by specifying only the beginning of the range (X..max).
	/// The range will only be partially checked.
	#[inline(always)]
	pub const fn from_startpos(start_pos: N) -> Self {
		const_new(start_pos, IgnoreNum)
	}
}

impl<N> FlockRangeEndPos<N>
where
	N: FlockRangePNumBeh,
{
	/// Create a range specifying only the end of the range (min..X).
	/// The range will only be partially checked.
	#[inline(always)]
	pub const fn from_endpos(end_pos: N) -> Self {
		const_new(IgnoreNum, end_pos)
	}
}

impl<N> FlockRangeCore<N>
where
	N: FlockRangePNumBeh,
{
	/// Set the start and end of the range using a primitive from the core.
	/// The range will be checked completely.
	#[inline(always)]
	pub const fn from_core(range: Range<N>) -> FlockRangeExp<N, N> {
		const_new(range.start, range.end)
	}
}

impl<NS: FlockRangePNumBeh, NE: FlockRangePNumBeh> FlockRangeExp<NS, NE> {
	/// Create a range of two numbers or their attributes.
	/// The range will be checked completely.
	#[inline]
	pub const fn new(start: NS, end: NE) -> FlockRangeExp<NS, NE> {
		const_new(start, end)
	}

	/// Create an unsafe range. Values in the range will never be validated.
	#[inline]
	pub const unsafe fn new_ignore_range_check(start: NS, end: NE) -> FlockRangeExpForce<NS, NE> {
		const_new(start, end).into_ignore_rangechecker()
	}
}

impl<NS, NE, C> FlockRange<NS, NE, C>
where
	NS: FlockRangePNumBeh,
	NE: FlockRangePNumBeh,
	C: FlockRangePNumBehChecker,
{
	/// Whether the range will be checked
	#[inline(always)]
	pub const fn is_en_range_check(&self) -> bool {
		C::IS_EN_RANGECHECK
	}

	/// Convert range to primitive range.
	#[inline]
	pub fn into_primitive(self) -> FlockRangePrimitive {
		let (start, end) = self.get();

		const_new(start, end)
	}

	#[inline(always)]
	pub const fn as_start(&self) -> &NS {
		&self.start
	}

	#[inline(always)]
	pub const fn as_end(&self) -> &NE {
		&self.end
	}

	/// Whether the range is just a position.
	#[inline(always)]
	pub const fn is_one_pos(&self) -> bool {
		NS::IS_ONEPOS
	}

	/// Whether the range is just a position.
	#[inline(always)]
	fn __is_one_pos(&self) -> bool {
		let result = self.is_one_pos();

		if result {
			debug_assert!(!(NE::IS_ONEPOS || NE::IS_IGNORE_NUM || NE::IS_FULLRANGE));
		}

		result
	}

	/// The range is full, range positions are calculated automatically.
	#[inline(always)]
	fn __is_full_range(&self) -> bool {
		let result = NS::IS_FULLRANGE;

		if result {
			debug_assert!(NE::IS_ONEPOS || NE::IS_IGNORE_NUM);
		}

		result
	}

	/// Calculate positions and get range from core.
	#[inline]
	pub fn into_core(self) -> Range<FlockRangePNum> {
		let (start, end) = self.get();

		Range { start, end }
	}

	/// Is the range correct.
	#[inline]
	pub fn is_valid_range(&self) -> bool
	where
		NE: PartialOrd<NS>,
	{
		self.check_range_fn(|| true, |_| false)
	}

	/// Get the position of the range if the range consists of a single number.
	#[inline]
	pub fn get_sys_one_pos(&self) -> Option<FlockRangePNum> {
		match self.__is_one_pos() {
			false => None,
			true => Some(self.end.get_pnum()),
		}
	}

	/// Calculate range and return platform numbers.
	#[inline]
	pub fn get(&self) -> (FlockRangePNum, FlockRangePNum) {
		//debug_assert_eq!(self.is_valid_range(), true);

		if self.__is_one_pos() {
			// onepos

			return (self.end.get_pnum(), self.end.get_pnum());
		}
		if self.__is_full_range() {
			// full

			return (NS::get_pnum_min(), NS::get_pnum_max());
		}

		match (NS::IS_IGNORE_NUM, NE::IS_IGNORE_NUM) {
			(true, true) => unreachable!(),

			/* A..=max */
			(false, true) => (self.start.get_pnum(), NS::get_pnum_max()),
			/* min..=A */
			(true, false) => (NE::get_pnum_min(), self.end.get_pnum()),
			/* A..=B */
			(false, false) => (self.start.get_pnum(), self.end.get_pnum()),
		}
	}

	#[inline]
	pub fn check_range_and_get(
		&self,
	) -> Result<(FlockRangePNum, FlockRangePNum), FlockRangeErr<NS, NE>>
	where
		NE: PartialOrd<NS>,
	{
		self.check_range_and_get_fn(|a, b| Ok((a, b)), Err)
	}

	/// Check the range for correctness.
	#[inline]
	pub fn check_range(&self) -> Result<(), FlockRangeErr<NS, NE>>
	where
		NE: PartialOrd<NS>,
	{
		self.check_range_fn(|| Ok(()), Err)
	}

	/// Check the range for correctness.
	#[inline]
	pub fn check_range_fn<R>(
		&self,
		next: impl FnOnce() -> R,
		err: impl FnOnce(FlockRangeErr<NS, NE>) -> R,
	) -> R
	where
		NE: PartialOrd<NS>,
	{
		match self.is_en_range_check() {
			false => next(),
			true => {
				if self.__is_one_pos() {
					// onepos
					let result = match self.end.is_valid() {
						true => next(),
						false => err(FlockRangeErr::InvalidShortRange(self.end)),
					};

					return result;
				}
				if self.__is_full_range() {
					// full
					// full ignore check.
					return next();
				}

				match (NS::IS_IGNORE_NUM, NE::IS_IGNORE_NUM) {
					(true, true) => unreachable!(),

					(false, true) =>
					/* A..=max */
					{
						match self.start.is_valid() {
							true if self.start.is_valid_endrange(NS::MAX) => next(),
							true => err(FlockRangeErr::InvalidStartEndPos3(self.start, NS::MAX)),
							false => err(FlockRangeErr::InvalidStartPos(self.start)),
						}
					}
					(true, false) =>
					/* min..=A */
					{
						match self.end.is_valid() {
							true if self.end.is_valid_endrange(NE::MAX) => next(),
							true => err(FlockRangeErr::InvalidStartEndPos2(NE::MIN, self.end)),
							false => err(FlockRangeErr::InvalidEndPos(self.end)),
						}
					}
					(false, false) =>
					/* A..=B */
					{
						match self.start.is_valid() {
							true => match self.end.is_valid() {
								true if self.start.is_valid_endrange(self.end) => next(),
								true => {
									err(FlockRangeErr::InvalidStartEndPos(self.start, self.end))
								}
								false => err(FlockRangeErr::InvalidEndPos(self.end)),
							},
							false => err(FlockRangeErr::InvalidStartPos(self.start)),
						}
					}
				}
			}
		}
	}

	/// Check the range for correctness and calculate the range.
	#[inline]
	pub fn check_range_and_get_fn<R>(
		&self,
		next: impl FnOnce(FlockRangePNum, FlockRangePNum) -> R,
		err: impl FnOnce(FlockRangeErr<NS, NE>) -> R,
	) -> R
	where
		NE: PartialOrd<NS>,
	{
		match self.is_en_range_check() {
			false => {
				if self.__is_one_pos() {
					// onepos
					return next(self.end.get_pnum(), self.end.get_pnum());
				}
				if self.__is_full_range() {
					// full
					return next(NS::get_pnum_min(), NS::get_pnum_max());
				}

				match (NS::IS_IGNORE_NUM, NE::IS_IGNORE_NUM) {
					(true, true) => unreachable!(),

					(false, true) =>
					/* A..=max */
					{
						next(self.start.get_pnum(), NS::get_pnum_max())
					}
					(true, false) =>
					/* min..=A */
					{
						next(NE::get_pnum_min(), self.end.get_pnum())
					}
					(false, false) =>
					/* A..=B */
					{
						next(self.start.get_pnum(), self.end.get_pnum())
					}
				}
			}
			true => {
				if self.__is_one_pos() {
					// onepos
					let result = match self.end.is_valid() {
						true => next(self.end.get_pnum(), self.end.get_pnum()),
						false => err(FlockRangeErr::InvalidShortRange(self.end)),
					};

					return result;
				}
				if self.__is_full_range() {
					// full
					// full ignore check.
					return next(NS::get_pnum_min(), NS::get_pnum_max());
				}

				match (NS::IS_IGNORE_NUM, NE::IS_IGNORE_NUM) {
					(true, true) => unreachable!(),

					(false, true) =>
					/* A..=max */
					{
						match self.start.is_valid() {
							true if self.start.is_valid_endrange(NS::MAX) => {
								next(self.start.get_pnum(), NS::get_pnum_max())
							}
							true => err(FlockRangeErr::InvalidStartEndPos3(self.start, NS::MAX)),
							false => err(FlockRangeErr::InvalidStartPos(self.start)),
						}
					}
					(true, false) =>
					/* min..=A */
					{
						match self.end.is_valid() {
							true if self.end.is_valid_endrange(NE::MAX) => {
								next(NE::get_pnum_min(), self.end.get_pnum())
							}
							true => err(FlockRangeErr::InvalidStartEndPos2(NE::MIN, self.end)),
							false => err(FlockRangeErr::InvalidEndPos(self.end)),
						}
					}
					(false, false) =>
					/* A..=B */
					{
						match self.start.is_valid() {
							true => match self.end.is_valid() {
								true if self.start.is_valid_endrange(self.end) => {
									next(self.start.get_pnum(), self.end.get_pnum())
								}
								true => {
									err(FlockRangeErr::InvalidStartEndPos(self.start, self.end))
								}
								false => err(FlockRangeErr::InvalidEndPos(self.end)),
							},
							false => err(FlockRangeErr::InvalidStartPos(self.start)),
						}
					}
				}
			}
		}
	}
}

impl<NS, NE, C, NS2, NE2, C2> PartialEq<FlockRange<NS2, NE2, C2>> for FlockRange<NS, NE, C>
where
	NS: FlockRangePNumBeh,
	NE: FlockRangePNumBeh,
	C: FlockRangePNumBehChecker,
	NS2: FlockRangePNumBeh,
	NE2: FlockRangePNumBeh,
	C2: FlockRangePNumBehChecker,
{
	#[inline]
	fn eq(&self, range2: &FlockRange<NS2, NE2, C2>) -> bool {
		/*if
			NS::IS_IGNORE_NUM != NS2::IS_IGNORE_NUM ||
			NE::IS_ONEPOS != NE2::IS_ONEPOS ||
			C::IS_EN_RANGECHECK != C2::IS_EN_RANGECHECK
		{
			return false;
		}

		todo!();*/
		let a1 = self.get();
		let a2 = range2.get();

		PartialEq::eq(&a1, &a2)
	}
}

impl<NS, NE, C, NS2, NE2, C2> PartialOrd<FlockRange<NS2, NE2, C2>> for FlockRange<NS, NE, C>
where
	NS: FlockRangePNumBeh,
	NE: FlockRangePNumBeh,
	C: FlockRangePNumBehChecker,
	NS2: FlockRangePNumBeh,
	NE2: FlockRangePNumBeh,
	C2: FlockRangePNumBehChecker,
{
	#[inline]
	fn partial_cmp(&self, range2: &FlockRange<NS2, NE2, C2>) -> Option<Ordering> {
		let a1 = self.get();
		let a2 = range2.get();

		PartialOrd::partial_cmp(&a1, &a2)
	}
}

#[test]
#[cfg(test)]
fn test_range() {
	use crate::range::pnum::__make_auto_pnum_type;

	impl FlockRangePNumBeh for i8 {
		const MIN: i8 = 0;
		const MAX: i8 = 16;

		#[inline(always)]
		fn get_pnum(self) -> FlockRangePNum {
			self as _
		}
	}
	__make_auto_pnum_type!(i8);

	{
		// nums
		assert!(!(-1i8).is_valid());
		assert!(0i8.is_valid());
		assert!(16i8.is_valid());
		assert!(!17i8.is_valid());
	}

	{
		// def core range
		{
			// 0..10
			let valid_range0 = FlockRange::from(0i8..10);
			assert!(valid_range0.is_en_range_check());
			assert!(valid_range0.is_valid_range());

			// 0..10
			let valid_range1 = FlockRange::from((0i8, 10));
			assert!(valid_range1.is_en_range_check());
			assert!(valid_range1.is_valid_range());

			assert!(valid_range0 == valid_range1);
		}

		// 0..max+1
		let invrange0 = FlockRange::from(0..<i8 as FlockRangePNumBeh>::MAX + 1);
		assert!(invrange0.is_en_range_check());
		assert!(!invrange0.is_valid_range());
	}

	{
		// one pos
		{
			// pos: 0
			let range0 = FlockRange::from(0i8);
			assert!(range0.is_en_range_check());
			assert!(range0.is_valid_range());

			// pos: 0
			let range1 = FlockRange::new_one_position(0i8);
			assert!(range1.is_en_range_check());
			assert!(range1.is_valid_range());

			assert!(range0 == range1);
		}

		// pos: max+1
		let invrange1 = FlockRange::new_one_position(<i8 as FlockRangePNumBeh>::MAX + 1);
		assert!(invrange1.is_en_range_check());
		assert!(!invrange1.is_valid_range());
	}

	{
		// full
		{
			// min..max
			let range0 = FlockRangeFull::<i8>::full();
			assert!(
				range0.get()
					== (
						<i8 as FlockRangePNumBeh>::MIN as _,
						<i8 as FlockRangePNumBeh>::MAX as _
					)
			);
			assert!(!range0.is_en_range_check()); // FALSE!
			assert!(range0.is_valid_range());

			// min..max
			let range1 = FlockRange::new(
				<i8 as FlockRangePNumBeh>::MIN,
				<i8 as FlockRangePNumBeh>::MAX,
			);
			assert!(range1.is_en_range_check());
			assert!(range1.is_valid_range());

			assert!(range0 == range1);

			// min..max
			let range3 = FlockRangeFull::<i8>::from(..);
			assert!(
				range0.get()
					== (
						<i8 as FlockRangePNumBeh>::MIN as _,
						<i8 as FlockRangePNumBeh>::MAX as _
					)
			);
			assert!(!range0.is_en_range_check()); // FALSE!
			assert!(range0.is_valid_range());

			assert!(range0 == range3);
		}

		// max..min
		let invrange1 = FlockRange::new(
			<i8 as FlockRangePNumBeh>::MAX,
			<i8 as FlockRangePNumBeh>::MIN,
		);
		assert!(invrange1.is_en_range_check());
		assert!(!invrange1.is_valid_range());
	}

	{
		// start_range
		// 10..
		let range0 = FlockRange::from(10i8..);

		let (a0, b0) = range0.get();
		assert_eq!(a0, 10i8 as FlockRangePNum);
		assert_eq!(b0, <i8 as FlockRangePNumBeh>::MAX as FlockRangePNum);

		assert!(range0.is_en_range_check());
		assert!(range0.is_valid_range());

		// ..10
		let range0 = unsafe { range0.into_ignore_rangechecker() };
		let (a0, b0) = range0.get();
		assert_eq!(a0, 10i8 as FlockRangePNum);
		assert_eq!(b0, <i8 as FlockRangePNumBeh>::MAX as FlockRangePNum);

		assert!(!range0.is_en_range_check());
		assert!(range0.is_valid_range()); // todo always true
	}

	{
		// end_range
		let range0 = FlockRange::from(..10i8);

		// min..10
		let (a0, b0) = range0.get();
		assert_eq!(a0, <i8 as FlockRangePNumBeh>::MIN as FlockRangePNum);
		assert_eq!(b0, 10);

		assert!(range0.is_en_range_check());
		assert!(range0.is_valid_range());

		// min..10
		let range0 = unsafe { range0.into_ignore_rangechecker() };
		let (a0, b0) = range0.get();
		assert_eq!(a0, <i8 as FlockRangePNumBeh>::MIN as FlockRangePNum);
		assert_eq!(b0, 10);

		assert!(!range0.is_en_range_check());
		assert!(range0.is_valid_range());
	}
}
