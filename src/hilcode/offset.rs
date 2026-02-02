use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Result;
use ::std::ops::Range;
use ::std::ops::RangeFrom;

pub(crate) trait Advance<DELTA> {
	fn advance(
		self: &Self,
		delta: DELTA,
	) -> Self;
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct RelOffset(usize);

impl Advance<usize> for RelOffset {
	fn advance(
		self: &Self,
		delta: usize,
	) -> Self {
		Self(self.0 + delta)
	}
}

impl RelOffset {
	pub(crate) const ZERO: Self = Self(0);

	pub(crate) fn to_range_from(self: &Self) -> RangeFrom<usize> {
		self.0..
	}

	pub(crate) fn to_range_up_to(self: &Self) -> Range<usize> {
		0..self.0
	}
}

#[cfg(test)]
impl RelOffset {
	pub(crate) const ONE: Self = Self(1);

	pub fn new(value: usize) -> Self {
		Self(value)
	}
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AbsOffset(usize);

impl Advance<usize> for AbsOffset {
	fn advance(
		self: &Self,
		delta: usize,
	) -> Self {
		Self(self.0 + delta)
	}
}

impl Advance<RelOffset> for AbsOffset {
	fn advance(
		self: &Self,
		delta: RelOffset,
	) -> Self {
		Self(self.0 + delta.0)
	}
}

#[coverage(off)]
impl Display for AbsOffset {
	fn fmt(
		&self,
		formatter: &mut Formatter<'_>,
	) -> Result {
		write!(formatter, "{}", self.0)
	}
}

impl AbsOffset {
	pub(crate) const ZERO: Self = Self(0);
}

#[cfg(test)]
impl AbsOffset {
	pub(crate) const ONE: Self = Self(1);

	pub fn new(value: usize) -> Self {
		Self(value)
	}
}
