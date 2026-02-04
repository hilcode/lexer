use crate::hilcode::offset::RelOffset;
use crate::hilcode::token_definition::TokenDefinition;

#[derive(Debug, PartialEq)]
pub(crate) struct TokenFound<TOKEN> {
	token: TOKEN,
	offset: RelOffset,
}

impl<TOKEN> TokenFound<TOKEN>
where
	TOKEN: TokenDefinition,
{
	pub(crate) fn new(
		token: TOKEN,
		offset: RelOffset,
	) -> TokenFound<TOKEN> {
		TokenFound { token, offset }
	}

	pub(crate) fn token(self: &Self) -> TOKEN {
		self.token.clone()
	}

	pub(crate) fn offset(self: &Self) -> RelOffset {
		self.offset
	}
}
