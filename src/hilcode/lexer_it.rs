use crate::hilcode::Lexer;
use crate::hilcode::fiber::Fiber;
use crate::hilcode::id::Id;
use crate::hilcode::offset::AbsOffset;
use crate::hilcode::offset::Advance;
use crate::hilcode::offset::RelOffset;
use crate::hilcode::positions::Positions;
use crate::hilcode::positions::StartPos;
use crate::hilcode::token_definition::TokenDefinition;
use crate::hilcode::token_found::TokenFound;
use ::imstr::ImString;
use ::std::collections::BTreeSet;

pub struct LexerIt<'lexer, TOKEN>
where
	TOKEN: TokenDefinition,
{
	lexer: &'lexer Lexer<TOKEN>,
	source: ImString,
	offset: AbsOffset,
}

impl<'lexer, TOKEN> LexerIt<'lexer, TOKEN>
where
	TOKEN: TokenDefinition,
{
	pub(crate) fn new(
		lexer: &'lexer Lexer<TOKEN>,
		source: ImString,
	) -> LexerIt<'lexer, TOKEN> {
		let offset: AbsOffset = AbsOffset::ZERO;
		LexerIt { lexer, source, offset }
	}

	pub(crate) fn start_ids(self: &Self) -> &Positions<StartPos> {
		&self.lexer.start_ids
	}

	pub(crate) fn step(
		self: &Self,
		token_found: &mut Option<TokenFound<TOKEN>>,
		active_fibers: BTreeSet<Fiber>,
	) -> BTreeSet<Fiber> {
		self.lexer.step(&self.source, self.offset, token_found, active_fibers)
	}

	pub(crate) fn end_of_source(self: &Self) -> bool {
		self.source.is_empty()
	}

	pub(crate) fn advance(
		self: &mut Self,
		offset: RelOffset,
	) {
		self.source = self.source.slice(offset.to_range_from());
		self.offset = self.offset.advance(offset);
	}

	pub(crate) fn advance_source(
		self: &mut Self,
		byte_count: usize,
	) {
		self.source = self.source.slice(byte_count..);
		self.offset = self.offset.advance(byte_count);
	}
}

impl<'lexer, TOKEN> Iterator for LexerIt<'lexer, TOKEN>
where
	TOKEN: TokenDefinition<This = TOKEN>,
{
	type Item = TOKEN;

	fn next(self: &mut Self) -> Option<Self::Item> {
		if self.end_of_source() {
			return None;
		}
		let mut maybe_token_found: Option<TokenFound<TOKEN>> = None;
		let mut active_fibers: BTreeSet<Fiber> = BTreeSet::new();
		self.start_ids().for_each(|start_id: Id| {
			let fiber: Fiber = Fiber::new(start_id, RelOffset::ZERO);
			active_fibers.insert(fiber);
		});
		loop {
			active_fibers = self.step(&mut maybe_token_found, active_fibers);
			if active_fibers.is_empty() {
				break;
			}
		}
		match maybe_token_found {
			Some(token_found) => {
				self.advance(token_found.offset());
				Some(token_found.token())
			}

			None => {
				let message: ImString = "No valid token found".into();
				let first_char: char = self.source.chars().next().unwrap();
				let skipped_text: ImString = ImString::from(first_char);
				let invalid_text: ImString = skipped_text.clone();
				let error_token: Self::Item = Self::Item::error(message, self.offset, &invalid_text);
				match self.source.chars().nth(1) {
					Some(next_char) => {
						self.advance_source(next_char.len_utf8());
					}

					None => {
						self.advance_source(1);
					}
				}
				Some(error_token)
			}
		}
	}
}
