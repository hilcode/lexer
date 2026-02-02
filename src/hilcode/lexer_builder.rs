use crate::hilcode::Lexer;
use crate::hilcode::id_flag::HasValidId;
use crate::hilcode::id_flag::NoId;
use crate::hilcode::id_provider::IdProvider;
use crate::hilcode::id_provider::IdProviderState;
use crate::hilcode::lexer_step::LexerStep;
use crate::hilcode::node;
use crate::hilcode::node::Node;
use crate::hilcode::node::generate_follow_pos;
use crate::hilcode::positions::Positions;
use crate::hilcode::positions::StartPos;
use crate::hilcode::token_builder::TokenBuilder;
use crate::hilcode::token_definition::TokenDefinition;
use crate::hilcode::token_id::TokenId;
use ::std::marker::PhantomData;

pub struct LexerBuilderEmpty<TOKEN>(PhantomData<TOKEN>);

impl<TOKEN> Default for LexerBuilderEmpty<TOKEN> {
	fn default() -> Self {
		Self(PhantomData)
	}
}

impl<TOKEN> LexerBuilderEmpty<TOKEN> {
	pub fn new_token(
		self: Self,
		node: Node<NoId>,
		to_token: TokenBuilder<TOKEN>,
	) -> LexerBuilder<TOKEN>
	where
		TOKEN: TokenDefinition,
	{
		let mut errors: Vec<String> = Vec::new();
		if node.nullable() {
			errors.push("Token #1 has a nullable definition".into());
		}
		let token_builders: Vec<TokenBuilder<TOKEN>> = vec![to_token];
		LexerBuilder::new(node.set_success_token_id(TokenId::ZERO), errors, token_builders)
	}
}

pub struct LexerBuilder<TOKEN> {
	root: Node<NoId>,
	errors: Vec<String>,
	token_builders: Vec<TokenBuilder<TOKEN>>,
}

impl<TOKEN> LexerBuilder<TOKEN>
where
	TOKEN: TokenDefinition,
{
	fn new(
		root: Node<NoId>,
		errors: Vec<String>,
		token_builders: Vec<TokenBuilder<TOKEN>>,
	) -> LexerBuilder<TOKEN> {
		LexerBuilder {
			root,
			errors,
			token_builders,
		}
	}

	pub fn new_token(
		self: Self,
		node: Node<NoId>,
		to_token: TokenBuilder<TOKEN>,
	) -> Self {
		let mut errors: Vec<String> = self.errors;
		if node.nullable() {
			let token_number: usize = self.token_builders.len() + 1;
			let error: String = format_args!("Token #{} has a nullable definition", token_number).to_string();
			errors.push(error);
		}
		let token_id: TokenId = TokenId::new(self.token_builders.len());
		let node: Node<NoId> = node.set_success_token_id(token_id);
		let root: Node<NoId> = node::one_of(self.root, node);
		let mut token_builders: Vec<TokenBuilder<TOKEN>> = self.token_builders;
		token_builders.push(to_token);
		Self {
			root,
			errors,
			token_builders,
		}
	}

	pub fn build(self: Self) -> Result<Lexer<TOKEN>, Vec<String>> {
		if self.errors.is_empty() {
			let mut state: IdProviderState = IdProviderState::default();
			let root: Node<HasValidId> = self.root.set_id(&mut state);
			let start_ids: Positions<StartPos> = root.get_start_ids();
			generate_follow_pos(&mut state, &root);
			let lexer_steps: Box<[LexerStep]> = state.to_lexer_steps();
			Result::Ok(Lexer::new(start_ids, lexer_steps, self.token_builders))
		} else {
			Result::Err(self.errors)
		}
	}
}
