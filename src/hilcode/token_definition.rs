use ::imstr::ImString;
use ::std::fmt::Debug;

pub trait TokenDefinition: Debug + Clone + Default + Ord {
	type This;

	fn error(message: impl Into<ImString>) -> Self::This;
}
