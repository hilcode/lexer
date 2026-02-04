use crate::hilcode::Lexer;
use crate::hilcode::fiber::Fiber;
use crate::hilcode::id::Id;
use crate::hilcode::lexer_error::LexerError;
use crate::hilcode::lexer_error::TokenCreationFailure;
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

	fn start_ids(self: &Self) -> &Positions<StartPos> {
		&self.lexer.start_ids
	}

	fn step(
		self: &Self,
		token_found: &mut Option<TokenFound<TOKEN>>,
		active_fibers: BTreeSet<Fiber>,
	) -> Result<BTreeSet<Fiber>, TokenCreationFailure> {
		self.lexer.step(&self.source, self.offset, token_found, active_fibers)
	}

	fn end_of_source(self: &Self) -> bool {
		self.source.is_empty()
	}

	fn advance(
		self: &mut Self,
		offset: RelOffset,
	) {
		self.source = self.source.slice(offset.to_range_from());
		self.offset = self.offset.advance(offset);
	}

	fn advance_source(
		self: &mut Self,
		byte_count: usize,
	) {
		self.source = self.source.slice(byte_count..);
		self.offset = self.offset.advance(byte_count);
	}

	fn initialize(self: &Self) -> BTreeSet<Fiber> {
		let mut active_fibers: BTreeSet<Fiber> = BTreeSet::new();
		self.start_ids().for_each(|start_id: Id| {
			let fiber: Fiber = Fiber::new(start_id, RelOffset::ZERO);
			active_fibers.insert(fiber);
		});
		active_fibers
	}
}

impl<'lexer, TOKEN> Iterator for LexerIt<'lexer, TOKEN>
where
	TOKEN: TokenDefinition<This = TOKEN>,
{
	type Item = Result<TOKEN, LexerError>;

	fn next(self: &mut Self) -> Option<Self::Item> {
		if self.end_of_source() {
			return None;
		}
		let mut maybe_token_found: Option<TokenFound<TOKEN>> = None;
		let mut active_fibers: BTreeSet<Fiber> = self.initialize();
		loop {
			match self.step(&mut maybe_token_found, active_fibers) {
				Result::Ok(new_active_fibers) => {
					if new_active_fibers.is_empty() {
						break;
					}
					active_fibers = new_active_fibers;
				}

				Result::Err(token_creation_failure) => {
					let lexer_error: LexerError = token_creation_failure.clone().into();
					self.advance_source(token_creation_failure.invalid_text.len());
					return Some(Result::Err(lexer_error));
				}
			}
		}
		match maybe_token_found {
			Some(token_found) => {
				self.advance(token_found.offset());
				Some(Result::Ok(token_found.token()))
			}

			None => {
				let first_char: char = self.source.chars().next().unwrap();
				let invalid_text: ImString = ImString::from(first_char);
				let invalid_text_size: usize = invalid_text.len();
				let offset_into_source: AbsOffset = self.offset;
				let lexer_error: LexerError = LexerError::NoValidTokenFound {
					offset_into_source,
					invalid_text,
					description: "No valid token found".into(),
				};
				self.advance_source(invalid_text_size);
				Some(Result::Err(lexer_error))
			}
		}
	}
}
