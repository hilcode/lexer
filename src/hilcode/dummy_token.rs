#[cfg(test)]
use crate::hilcode::token_definition::TokenDefinition;
#[cfg(test)]
use ::imstr::ImString;

#[cfg(test)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum DummyToken {
	Error(ImString),
	Default(ImString),
	Asterisk(ImString),
	Best(ImString),
}

#[cfg(test)]
impl Default for DummyToken {
	fn default() -> DummyToken {
		DummyToken::Error(ImString::from("Default Token"))
	}
}

#[cfg(test)]
#[coverage(off)]
pub(crate) fn to_asterisk(matched_text: &ImString) -> DummyToken {
	DummyToken::Asterisk(matched_text.clone())
}

#[cfg(test)]
#[coverage(off)]
pub(crate) fn to_default(matched_text: &ImString) -> DummyToken {
	DummyToken::Default(matched_text.clone())
}

#[cfg(test)]
#[coverage(off)]
pub(crate) fn to_best(matched_text: &ImString) -> DummyToken {
	DummyToken::Best(matched_text.clone())
}

#[cfg(test)]
#[coverage(off)]
impl TokenDefinition for DummyToken {
	type This = Self;

	fn error(message: impl Into<ImString>) -> Self::This {
		DummyToken::Error(message.into())
	}
}
