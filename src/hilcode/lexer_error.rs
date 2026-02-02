use crate::hilcode::offset::AbsOffset;
use ::imstr::ImString;
use ::std::error::Error;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Result;

#[derive(Clone, Debug, PartialEq)]
pub struct TokenCreationFailure {
	pub offset_into_source: AbsOffset,
	pub invalid_text: ImString,
	pub description: String,
}

#[derive(Debug, Eq, PartialEq)]
pub enum LexerError {
	TokenCreationFailure {
		offset_into_source: AbsOffset,
		invalid_text: ImString,
		description: String,
	},
	NoValidTokenFound {
		offset_into_source: AbsOffset,
		invalid_text: ImString,
		description: String,
	},
}

impl From<TokenCreationFailure> for LexerError {
	fn from(value: TokenCreationFailure) -> Self {
		LexerError::TokenCreationFailure {
			offset_into_source: value.offset_into_source,
			invalid_text: value.invalid_text,
			description: value.description,
		}
	}
}

#[coverage(off)]
impl Display for LexerError {
	fn fmt(
		self: &Self,
		formatter: &mut Formatter<'_>,
	) -> Result {
		match self {
			LexerError::TokenCreationFailure {
				offset_into_source: _,
				invalid_text,
				description,
			} => {
				write!(formatter, "{}: \"{}\"", description, invalid_text)
			}

			LexerError::NoValidTokenFound {
				offset_into_source: _,
				invalid_text,
				description,
			} => {
				write!(formatter, "{}: \"{}\"", description, invalid_text)
			}
		}
	}
}

impl Error for LexerError {
	// Empty
}
