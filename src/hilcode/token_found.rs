#[derive(Debug, PartialEq)]
pub(crate) struct TokenFound<TOKEN> {
	token: TOKEN,
	offset: usize,
}

impl<TOKEN> TokenFound<TOKEN>
where
	TOKEN: Clone + PartialEq,
{
	pub(crate) fn new(
		token: TOKEN,
		offset: usize,
	) -> TokenFound<TOKEN> {
		TokenFound { token, offset }
	}

	pub(crate) fn token(self: &Self) -> TOKEN {
		self.token.clone()
	}

	pub(crate) fn offset(self: &Self) -> usize {
		self.offset
	}
}
