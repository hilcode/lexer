pub(crate) mod dummy_token;
pub(crate) mod fiber;
pub(crate) mod id_flag;
pub(crate) mod id_provider;
pub(crate) mod lexer_builder;
pub mod lexer_it;
pub(crate) mod lexer_step;
pub mod node;
pub(crate) mod node_type;
pub(crate) mod positions;
pub(crate) mod test_data;
pub(crate) mod token_builder;
pub mod token_definition;
pub(crate) mod token_found;
pub(crate) mod token_id;

use crate::hilcode::fiber::Fiber;
use crate::hilcode::lexer_builder::LexerBuilderEmpty;
use crate::hilcode::lexer_it::LexerIt;
use crate::hilcode::lexer_step::LexerStep;
use crate::hilcode::positions::Positions;
use crate::hilcode::positions::StartPos;
use crate::hilcode::token_builder::TokenBuilder;
use crate::hilcode::token_definition::TokenDefinition;
use crate::hilcode::token_found::TokenFound;
use ::imstr::ImString;
use ::std::collections::BTreeSet;

#[derive(Debug)]
pub struct Lexer<TOKEN>
where
	TOKEN: TokenDefinition,
{
	start_ids: Positions<StartPos>,
	lexer_steps: Box<[LexerStep]>,
	token_builders: Vec<TokenBuilder<TOKEN>>,
}

impl<TOKEN> Default for Lexer<TOKEN>
where
	TOKEN: TokenDefinition,
{
	fn default() -> Self {
		Self {
			start_ids: Default::default(),
			lexer_steps: Default::default(),
			token_builders: Default::default(),
		}
	}
}

impl<TOKEN> Lexer<TOKEN>
where
	TOKEN: TokenDefinition,
{
	pub fn builder() -> LexerBuilderEmpty<TOKEN> {
		LexerBuilderEmpty::default()
	}

	pub(crate) fn new(
		start_ids: Positions<StartPos>,
		lexer_steps: Box<[LexerStep]>,
		token_builders: Vec<TokenBuilder<TOKEN>>,
	) -> Lexer<TOKEN> {
		Lexer {
			start_ids,
			lexer_steps,
			token_builders,
		}
	}

	pub(crate) fn get_step(
		self: &Self,
		index: usize,
	) -> &LexerStep {
		unsafe { self.lexer_steps.get_unchecked(index) }
	}

	pub fn tokenize<'lexer>(
		self: &'lexer Self,
		source: impl Into<ImString>,
	) -> LexerIt<'lexer, TOKEN> {
		LexerIt::new(self, source.into())
	}

	pub(crate) fn step(
		self: &Self,
		source: &ImString,
		token_found: &mut Option<TokenFound<TOKEN>>,
		active_fibers: BTreeSet<Fiber>,
	) -> BTreeSet<Fiber> {
		let mut fibers: BTreeSet<Fiber> = BTreeSet::new();
		active_fibers.iter().for_each(|active_fiber: &Fiber| {
			active_fiber.run(token_found, &mut fibers, source, self);
		});
		fibers
	}
}

#[cfg(test)]
use crate::hilcode::token_id::TokenId;

#[cfg(test)]
#[coverage(off)]
impl<TOKEN> Lexer<TOKEN>
where
	TOKEN: TokenDefinition,
{
	pub(crate) fn _builder() -> LexerBuilder<TOKEN> {
		LexerBuilder::default()
	}
}

#[cfg(test)]
pub(crate) struct LexerBuilder<TOKEN> {
	start_ids: Positions<StartPos>,
	lexer_steps: Vec<LexerStep>,
	token_builders: Vec<TokenBuilder<TOKEN>>,
}

#[cfg(test)]
#[coverage(off)]
impl<TOKEN> Default for LexerBuilder<TOKEN> {
	fn default() -> Self {
		Self {
			start_ids: Default::default(),
			lexer_steps: Default::default(),
			token_builders: Default::default(),
		}
	}
}

#[cfg(test)]
#[coverage(off)]
impl<TOKEN> LexerBuilder<TOKEN>
where
	TOKEN: TokenDefinition,
{
	pub(crate) fn build(self: Self) -> Lexer<TOKEN> {
		Lexer::new(self.start_ids, self.lexer_steps.into_boxed_slice(), self.token_builders)
	}

	pub(crate) fn start_ids(
		mut self: Self,
		start_ids: &[usize],
	) -> Self {
		self.start_ids = Positions::<StartPos>::new(start_ids);
		self
	}

	pub(crate) fn lexer_step_success(
		mut self: Self,
		next: &[usize],
		expected: &str,
		to_token: TokenBuilder<TOKEN>,
	) -> Self {
		let success: TokenId = TokenId::new(self.token_builders.len());
		self.token_builders.push(to_token);
		let lexer_step: LexerStep = LexerStep::_builder().next(next).expected(expected).success(success).build();
		self.lexer_steps.push(lexer_step);
		self
	}

	pub(crate) fn lexer_step(
		mut self: Self,
		next: &[usize],
		expected: &str,
	) -> Self {
		let lexer_step: LexerStep = LexerStep::_builder().next(next).expected(expected).build();
		self.lexer_steps.push(lexer_step);
		self
	}
}

#[cfg(test)]
#[coverage(off)]
pub(crate) mod tests;
