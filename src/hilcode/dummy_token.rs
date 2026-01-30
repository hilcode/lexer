#[cfg(test)]
use crate::hilcode::offset::AbsOffset;
#[cfg(test)]
use crate::hilcode::token_definition::TokenDefinition;
#[cfg(test)]
use ::imstr::ImString;

#[cfg(test)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct DummyToken {
	token_type: DummyTokenType,
	start_offset: AbsOffset,
	text: ImString,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum DummyTokenType {
	Error(ImString),
	Default,
	Asterisk,
	Best,
}

#[cfg(test)]
#[coverage(off)]
pub(crate) fn to_asterisk(
	start_offset: AbsOffset,
	text: &ImString,
) -> DummyToken {
	DummyToken {
		start_offset,
		text: text.clone(),
		token_type: DummyTokenType::Asterisk,
	}
}

#[cfg(test)]
#[coverage(off)]
pub(crate) fn to_default(
	start_offset: AbsOffset,
	text: &ImString,
) -> DummyToken {
	DummyToken {
		start_offset,
		text: text.clone(),
		token_type: DummyTokenType::Default,
	}
}

#[cfg(test)]
#[coverage(off)]
pub(crate) fn to_best(
	start_offset: AbsOffset,
	text: &ImString,
) -> DummyToken {
	DummyToken {
		start_offset,
		text: text.clone(),
		token_type: DummyTokenType::Best,
	}
}

#[cfg(test)]
#[coverage(off)]
impl TokenDefinition for DummyToken {
	type This = Self;

	fn start_offset(self: &Self) -> AbsOffset {
		self.start_offset
	}

	fn text(self: &Self) -> &ImString {
		&self.text
	}

	fn error(
		message: impl Into<ImString>,
		start_offset: AbsOffset,
		text: &ImString,
	) -> Self::This {
		DummyToken {
			start_offset,
			text: text.clone(),
			token_type: DummyTokenType::Error(message.into()),
		}
	}
}
