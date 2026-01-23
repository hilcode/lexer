use crate::hilcode::Lexer;
use crate::hilcode::TokenFound;
use crate::hilcode::lexer_step::LexerStep;
use crate::hilcode::token_definition::TokenDefinition;
use ::imstr::ImString;
use ::std::collections::BTreeSet;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Fiber {
	position: usize,
	offset: usize,
}

impl Fiber {
	pub(crate) fn new(
		position: usize,
		offset: usize,
	) -> Fiber {
		Fiber { position, offset }
	}

	pub(crate) fn run<TOKEN>(
		self: &Self,
		token_found: &mut Option<TokenFound<TOKEN>>,
		fibers: &mut BTreeSet<Fiber>,
		source: &ImString,
		lexer: &Lexer<TOKEN>,
	) where
		TOKEN: TokenDefinition,
	{
		let lexer_step: &LexerStep = lexer.get_step(self.position);
		match lexer_step.matches(&source.slice(self.offset..)) {
			Some(matched_byte_count) => {
				let new_offset: usize = self.offset + matched_byte_count;
				if let Some(token_builder) = lexer_step.get_token_builder(lexer) {
					let token: TOKEN = token_builder(&source.slice(0..new_offset));
					match token_found {
						None => {
							token_found.replace(TokenFound::new(token, new_offset));
						}

						Some(current_token_found) => {
							let current_token: TOKEN = current_token_found.token();
							if current_token < token {
								token_found.replace(TokenFound::new(token, new_offset));
							}
						}
					}
				}
				lexer_step.next().for_each(|next_position: usize| {
					let fiber: Fiber = Fiber::new(next_position, new_offset);
					fibers.insert(fiber);
				});
			}

			None => {
				// Do nothing
			}
		}
	}
}

#[cfg(test)]
impl Fiber {
	pub(crate) fn _builder() -> FiberBuilder {
		FiberBuilder::default()
	}
}

#[cfg(test)]
#[derive(Default)]
pub(crate) struct FiberBuilder {
	position: usize,
	offset: usize,
}

#[cfg(test)]
#[coverage(off)]
impl FiberBuilder {
	pub(crate) fn build(self: Self) -> Fiber {
		Fiber::new(self.position, self.offset)
	}

	pub(crate) fn position(
		mut self: Self,
		position: usize,
	) -> Self {
		self.position = position;
		self
	}

	pub(crate) fn offset(
		mut self: Self,
		offset: usize,
	) -> Self {
		self.offset = offset;
		self
	}
}

#[cfg(test)]
#[coverage(off)]
mod tests;
