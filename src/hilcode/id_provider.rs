use crate::hilcode::lexer_step::LexerStep;
use crate::hilcode::token_id::TokenId;
use ::imstr::ImString;

pub(crate) trait IdProvider {
	fn get_next_id(self: &Self) -> usize;

	fn get_lexer_step_mut(
		self: &mut Self,
		id: usize,
	) -> &mut LexerStep;

	fn to_lexer_steps(self: Self) -> Box<[LexerStep]>;

	fn append(
		self: &mut Self,
		expected: ImString,
		success: Option<TokenId>,
	);
}

#[derive(Default)]
pub(crate) struct IdProviderState(Vec<LexerStep>);

impl IdProvider for IdProviderState {
	fn get_next_id(self: &Self) -> usize {
		self.0.len()
	}

	fn get_lexer_step_mut(
		self: &mut Self,
		id: usize,
	) -> &mut LexerStep {
		let follow_pos: &mut LexerStep = unsafe { self.0.get_unchecked_mut(id) };
		follow_pos
	}

	fn to_lexer_steps(self: Self) -> Box<[LexerStep]> {
		self.0.into_boxed_slice()
	}

	fn append(
		self: &mut Self,
		expected: ImString,
		success: Option<TokenId>,
	) {
		self.0.push(LexerStep::new(expected, success));
	}
}
