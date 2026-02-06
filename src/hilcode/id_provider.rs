use crate::hilcode::id::Id;
use crate::hilcode::lexer_step::LexerStep;
use crate::hilcode::success::Success;
use ::imstr::ImString;

pub(crate) trait IdProvider {
	fn get_next_id(self: &Self) -> Id;

	fn get_lexer_step_mut(
		self: &mut Self,
		id: Id,
	) -> &mut LexerStep;

	fn to_lexer_steps(self: Self) -> Box<[LexerStep]>;

	fn append(
		self: &mut Self,
		expected: ImString,
		success: Option<Success>,
	);
}

#[derive(Default)]
pub(crate) struct IdProviderState(Vec<LexerStep>);

impl IdProvider for IdProviderState {
	fn get_next_id(self: &Self) -> Id {
		Id::new(self.0.len())
	}

	fn get_lexer_step_mut(
		self: &mut Self,
		id: Id,
	) -> &mut LexerStep {
		let follow_pos: &mut LexerStep = unsafe { self.0.get_unchecked_mut(id.0) };
		follow_pos
	}

	fn to_lexer_steps(self: Self) -> Box<[LexerStep]> {
		self.0.into_boxed_slice()
	}

	fn append(
		self: &mut Self,
		expected: ImString,
		success: Option<Success>,
	) {
		self.0.push(LexerStep::new(expected, success));
	}
}
