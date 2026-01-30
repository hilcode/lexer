use crate::hilcode::Lexer;
use crate::hilcode::TokenFound;
use crate::hilcode::id::Id;
use crate::hilcode::lexer_step::LexerStep;
use crate::hilcode::offset::Advance;
use crate::hilcode::offset::RelOffset;
use crate::hilcode::token_definition::TokenDefinition;
use ::imstr::ImString;
use ::std::collections::BTreeSet;

#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Fiber {
	id: Id,
	offset: RelOffset,
}

impl Fiber {
	pub(crate) fn new(
		id: Id,
		offset: RelOffset,
	) -> Fiber {
		Fiber { id, offset }
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
		let lexer_step: &LexerStep = lexer.get_step(self.id);
		match lexer_step.matches(&source.slice(self.offset.to_range_from())) {
			Some(matched_byte_count) => {
				let new_offset: RelOffset = self.offset.advance(matched_byte_count);
				if let Some(token_builder) = lexer_step.get_token_builder(lexer) {
					let token: TOKEN = token_builder(&source.slice(new_offset.to_range_up_to()));
					Fiber::update_token_found(token_found, token, new_offset);
				}
				lexer_step.next().for_each(|next_id: Id| {
					let fiber: Fiber = Fiber::new(next_id, new_offset);
					fibers.insert(fiber);
				});
			}

			None => {
				// Do nothing
			}
		}
	}

	fn update_token_found<TOKEN>(
		token_found: &mut Option<TokenFound<TOKEN>>,
		token: TOKEN,
		new_offset: RelOffset,
	) where
		TOKEN: TokenDefinition,
	{
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
	id: Id,
	offset: RelOffset,
}

#[cfg(test)]
#[coverage(off)]
impl FiberBuilder {
	pub(crate) fn build(self: Self) -> Fiber {
		Fiber::new(self.id, self.offset)
	}

	pub(crate) fn id(
		mut self: Self,
		id: usize,
	) -> Self {
		self.id = Id::new(id);
		self
	}

	pub(crate) fn offset(
		mut self: Self,
		offset: usize,
	) -> Self {
		self.offset = RelOffset::new(offset);
		self
	}
}

#[cfg(test)]
#[coverage(off)]
mod tests;
