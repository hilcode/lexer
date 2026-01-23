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

#[cfg(test)]
impl<TOKEN> TokenFound<TOKEN>
where
	TOKEN: Default,
{
	pub(crate) fn _builder() -> TokenFoundBuilder<TOKEN> {
		TokenFoundBuilder::default()
	}
}

#[cfg(test)]
pub(crate) struct TokenFoundBuilder<TOKEN> {
	token: TOKEN,
	offset: usize,
}

#[cfg(test)]
impl<TOKEN> Default for TokenFoundBuilder<TOKEN>
where
	TOKEN: Default,
{
	fn default() -> Self {
		Self {
			token: Default::default(),
			offset: 0,
		}
	}
}

#[cfg(test)]
impl<TOKEN> TokenFoundBuilder<TOKEN> {
	pub(crate) fn build(self: Self) -> TokenFound<TOKEN> {
		TokenFound {
			token: self.token,
			offset: self.offset,
		}
	}

	pub(crate) fn token(
		mut self: Self,
		token: TOKEN,
	) -> Self {
		self.token = token;
		self
	}
}
