use crate::hilcode::Lexer;
use crate::hilcode::positions::FirstPos;
use crate::hilcode::positions::FollowPos;
use crate::hilcode::positions::Positions;
use crate::hilcode::token_builder::TokenBuilder;
use crate::hilcode::token_definition::TokenDefinition;
use crate::hilcode::token_id::TokenId;
use ::imstr::ImString;

#[derive(Debug)]
pub(crate) struct LexerStep {
	next: Positions<FollowPos>,
	expected: ImString,
	success: Option<TokenId>,
}

impl LexerStep {
	pub(crate) fn new(
		expected: ImString,
		success: Option<TokenId>,
	) -> LexerStep {
		LexerStep {
			next: Positions::default(),
			expected,
			success,
		}
	}

	pub(crate) fn get_token_builder<'token, TOKEN>(
		self: &Self,
		lexer: &'token Lexer<TOKEN>,
	) -> Option<&'token TokenBuilder<TOKEN>>
	where
		TOKEN: TokenDefinition,
	{
		self.success.map(|token_id: TokenId| -> &TokenBuilder<TOKEN> {
			return token_id.get(&lexer.token_builders).unwrap();
		})
	}

	pub(crate) fn next(self: &Self) -> &Positions<FollowPos> {
		&self.next
	}

	pub(crate) fn matches(
		self: &Self,
		source: &str,
	) -> Option<usize> {
		if source.starts_with(self.expected.as_str()) {
			Some(self.expected.len())
		} else {
			None
		}
	}

	pub(crate) fn append_to_follow_pos(
		self: &mut Self,
		positions: &Positions<FirstPos>,
	) {
		self.next.union_first_pos(positions);
	}
}

#[cfg(test)]
#[derive(Default)]
pub(crate) struct LexerStepBuilder {
	next: Positions<FollowPos>,
	expected: ImString,
	success: Option<TokenId>,
}

#[cfg(test)]
impl LexerStep {
	pub(crate) fn _builder() -> LexerStepBuilder {
		LexerStepBuilder::default()
	}
}

#[cfg(test)]
impl LexerStepBuilder {
	pub(crate) fn build(self: Self) -> LexerStep {
		LexerStep {
			next: self.next,
			expected: self.expected,
			success: self.success,
		}
	}

	pub(crate) fn next(
		mut self: Self,
		next: &[usize],
	) -> Self {
		self.next = Positions::new(next);
		self
	}

	pub(crate) fn expected(
		mut self: Self,
		expected: &str,
	) -> Self {
		self.expected = ImString::from(expected);
		self
	}

	pub(crate) fn success(
		mut self: Self,
		success: TokenId,
	) -> Self {
		self.success = Some(success);
		self
	}
}
