use crate::hilcode::Lexer;
use crate::hilcode::fiber::Fiber;
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
}

impl<'lexer, TOKEN> LexerIt<'lexer, TOKEN>
where
	TOKEN: TokenDefinition,
{
	pub(crate) fn new(
		lexer: &'lexer Lexer<TOKEN>,
		source: ImString,
	) -> LexerIt<'lexer, TOKEN> {
		LexerIt { lexer, source }
	}
}

impl<'lexer, TOKEN> Iterator for LexerIt<'lexer, TOKEN>
where
	TOKEN: TokenDefinition<This = TOKEN>,
{
	type Item = TOKEN;

	fn next(self: &mut Self) -> Option<Self::Item> {
		if self.source.is_empty() {
			return None;
		}
		let mut maybe_token_found: Option<TokenFound<TOKEN>> = None;
		let mut active_fibers: BTreeSet<Fiber> = BTreeSet::new();
		self.lexer.start_ids.for_each(|start_id: usize| {
			let fiber: Fiber = Fiber::new(start_id, 0);
			active_fibers.insert(fiber);
		});
		loop {
			active_fibers = self.lexer.step(&self.source, &mut maybe_token_found, active_fibers);
			if active_fibers.is_empty() {
				break;
			}
		}
		match maybe_token_found {
			Some(token_found) => {
				self.source = self.source.slice(token_found.offset()..);
				Some(token_found.token())
			}

			None => {
				let message: ImString = "No valid token found".into();
				let error_token: Self::Item = Self::Item::error(message);
				match self.source.chars().nth(1) {
					Some(next_char) => {
						self.source = self.source.slice(next_char.len_utf8()..);
					}

					None => {
						self.source = self.source.slice(1..);
					}
				}
				Some(error_token)
			}
		}
	}
}
