use crate::hilcode::token_id::TokenId;
use ::std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Success {
	token_id: TokenId,
	success_type: SuccessType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SuccessType {
	Default,
	IfFollowedBy,
	IfNotFollowedBy,
	Tail,
}

impl Success {
	pub(crate) fn new(token_id: TokenId) -> Success {
		Success {
			token_id,
			success_type: SuccessType::Default,
		}
	}

	pub(crate) fn token_id(self: &Self) -> TokenId {
		self.token_id
	}

	pub(crate) fn success_type(self: &Self) -> SuccessType {
		self.success_type
	}
}
