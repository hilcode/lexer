use crate::hilcode::offset::AbsOffset;
use ::imstr::ImString;

pub trait TokenDefinition: Clone + Ord {
	type This;

	fn start_offset(self: &Self) -> AbsOffset;

	fn text(self: &Self) -> &ImString;

	fn error(
		message: impl Into<ImString>,
		start_offset: AbsOffset,
		text: &ImString,
	) -> Self::This;
}
