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
