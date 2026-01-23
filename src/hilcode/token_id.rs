#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct TokenId(usize);

impl TokenId {
	pub(crate) const ZERO: TokenId = TokenId(0);

	pub(crate) fn new(token_id: usize) -> TokenId {
		TokenId(token_id)
	}

	pub(crate) fn get<'element, A>(
		self: &Self,
		vector: &'element [A],
	) -> Option<&'element A> {
		vector.get(self.0)
	}
}
