#[cfg(test)]
use crate::hilcode::Lexer;
#[cfg(test)]
use crate::hilcode::fiber::Fiber;
#[cfg(test)]
use crate::hilcode::token_definition::TokenDefinition;
#[cfg(test)]
use crate::hilcode::token_found::TokenFound;
#[cfg(test)]
use ::std::collections::BTreeSet;

#[cfg(test)]
#[derive(Debug)]
pub(crate) struct TestData<TOKEN>
where
	TOKEN: TokenDefinition,
{
	pub(crate) lexer: Lexer<TOKEN>,
	pub(crate) fiber: Fiber,
	pub(crate) token_found: Option<TokenFound<TOKEN>>,
	pub(crate) active_fibers: BTreeSet<Fiber>,
}

#[cfg(test)]
#[coverage(off)]
impl<TOKEN> TestData<TOKEN>
where
	TOKEN: TokenDefinition,
{
	pub(crate) fn _builder() -> TestDataBuilder<TOKEN> {
		TestDataBuilder::default()
	}
}

#[cfg(test)]
pub(crate) struct TestDataBuilder<TOKEN>
where
	TOKEN: TokenDefinition,
{
	lexer: Lexer<TOKEN>,
	active_fibers: BTreeSet<Fiber>,
	token_found: Option<TokenFound<TOKEN>>,
	fiber: Fiber,
}

#[cfg(test)]
#[coverage(off)]
impl<TOKEN> Default for TestDataBuilder<TOKEN>
where
	TOKEN: TokenDefinition,
{
	fn default() -> Self {
		Self {
			lexer: Default::default(),
			active_fibers: Default::default(),
			token_found: Default::default(),
			fiber: Default::default(),
		}
	}
}

#[cfg(test)]
#[coverage(off)]
impl<TOKEN> TestDataBuilder<TOKEN>
where
	TOKEN: TokenDefinition,
{
	pub(crate) fn build(self: Self) -> TestData<TOKEN> {
		TestData {
			lexer: self.lexer,
			active_fibers: self.active_fibers,
			token_found: self.token_found,
			fiber: self.fiber,
		}
	}

	pub(crate) fn lexer(
		mut self: Self,
		lexer: Lexer<TOKEN>,
	) -> Self {
		self.lexer = lexer;
		self
	}

	pub(crate) fn fiber(
		mut self: Self,
		fiber: Fiber,
	) -> Self {
		self.fiber = fiber;
		self
	}

	pub(crate) fn active_fiber(
		mut self: Self,
		active_fiber: Fiber,
	) -> Self {
		self.active_fibers.insert(active_fiber);
		self
	}

	pub(crate) fn token_found(
		mut self: Self,
		token_found: TokenFound<TOKEN>,
	) -> Self {
		self.token_found = Some(token_found);
		self
	}
}
