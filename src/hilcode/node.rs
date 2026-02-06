use crate::hilcode::id::Id;
use crate::hilcode::id_flag::HasValidId;
use crate::hilcode::id_flag::NoId;
use crate::hilcode::id_provider::IdProvider;
use crate::hilcode::lexer_step::LexerStep;
use crate::hilcode::node;
use crate::hilcode::node_type::NodeType;
use crate::hilcode::positions;
use crate::hilcode::positions::FirstPos;
use crate::hilcode::positions::LastPos;
use crate::hilcode::positions::Positions;
use crate::hilcode::positions::StartPos;
use crate::hilcode::success::Success;
use crate::hilcode::token_id::TokenId;
use ::imstr::ImString;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::marker::PhantomData;

#[derive(Clone, Eq, PartialEq)]
pub struct Node<ID>
where
	ID: Debug,
{
	nullable: bool,
	first_pos: Positions<FirstPos>,
	last_pos: Positions<LastPos>,
	node_type: NodeType<ID>,
	phantom: PhantomData<ID>,
}

#[coverage(off)]
impl<ID> Debug for Node<ID>
where
	ID: Debug,
{
	fn fmt(
		&self,
		formatter: &mut Formatter<'_>,
	) -> std::fmt::Result {
		formatter
			.debug_struct("Node")
			.field("nullable", &self.nullable)
			.field("first_pos", &self.first_pos)
			.field("last_pos", &self.last_pos)
			.field("node_type", &self.node_type)
			.finish()
	}
}

pub fn text(expected: impl Into<ImString>) -> Node<NoId> {
	Node::new(Node::NOT_NULLABLE, NodeType::terminal(expected.into()))
}

fn empty<ID>() -> Node<ID>
where
	ID: Debug,
{
	Node::new(Node::NULLABLE, NodeType::new_empty())
}

pub fn one_of(
	lhs: Node<NoId>,
	rhs: Node<NoId>,
) -> Node<NoId> {
	Node::new(lhs.nullable || rhs.nullable, NodeType::one_of(lhs, rhs))
}

pub fn repeat(node: Node<NoId>) -> Node<NoId> {
	Node::new(Node::NULLABLE, NodeType::repeat(node))
}

pub fn repeat1(node: Node<NoId>) -> Node<NoId> {
	concat(node.clone(), node::repeat(node))
}

pub fn optionally(node: Node<NoId>) -> Node<NoId> {
	one_of(node, empty())
}

fn concat(
	lhs: Node<NoId>,
	rhs: Node<NoId>,
) -> Node<NoId> {
	Node::new(lhs.nullable && rhs.nullable, NodeType::concat(lhs, rhs))
}

impl Node<NoId> {
	const NULLABLE: bool = true;
	const NOT_NULLABLE: bool = false;

	pub(crate) fn nullable(self: &Self) -> bool {
		self.nullable
	}

	pub fn text(
		self: Self,
		expected: impl Into<ImString>,
	) -> Node<NoId> {
		concat(self, text(expected))
	}

	pub fn one_of(
		self: Self,
		lhs: Node<NoId>,
		rhs: Node<NoId>,
	) -> Node<NoId> {
		concat(self, one_of(lhs, rhs))
	}

	pub fn repeat(
		self: Self,
		node: Node<NoId>,
	) -> Node<NoId> {
		concat(self, repeat(node))
	}

	pub fn repeat1(
		self: Self,
		node: Node<NoId>,
	) -> Node<NoId> {
		concat(self, repeat1(node))
	}

	pub fn optionally(
		self: Self,
		node: Node<NoId>,
	) -> Node<NoId> {
		concat(self, optionally(node))
	}

	pub(crate) fn set_success_token_id(
		self: &Self,
		token_id: TokenId,
	) -> Node<NoId> {
		match &self.node_type {
			NodeType::Empty => {
				return self.clone();
			}

			NodeType::Terminal { expected, success: _ } => {
				let success: Success = Success::new(token_id);
				self.with_node_type(NodeType::new_terminal_with_to_token(expected.clone(), Some(success)))
			}

			NodeType::Concat { lhs, rhs } => {
				let lhs: Box<Node<NoId>> = if rhs.nullable() {
					Box::new(lhs.set_success_token_id(token_id))
				} else {
					lhs.clone()
				};
				let rhs: Box<Node<NoId>> = Box::new(rhs.set_success_token_id(token_id));
				let node_type: NodeType<NoId> = NodeType::Concat { lhs, rhs };
				self.with_node_type(node_type)
			}

			NodeType::OneOf { lhs, rhs } => {
				let node_type: NodeType<NoId> = NodeType::OneOf {
					lhs: Box::new(lhs.set_success_token_id(token_id)),
					rhs: Box::new(rhs.set_success_token_id(token_id)),
				};
				self.with_node_type(node_type)
			}

			NodeType::Repeat { repeat } => {
				let node_type: NodeType<NoId> = NodeType::Repeat {
					repeat: Box::new(repeat.set_success_token_id(token_id)),
				};
				self.with_node_type(node_type)
			}
		}
	}

	pub(crate) fn set_id(
		self: &Self,
		state: &mut impl IdProvider,
	) -> Node<HasValidId> {
		match &self.node_type {
			NodeType::Empty => {
				return empty();
			}

			NodeType::Terminal { expected, success } => {
				let id: Id = state.get_next_id();
				let mut first_pos: Positions<FirstPos> = self.first_pos.clone();
				first_pos.insert(id);
				let last_pos: Positions<LastPos> = first_pos.to_last_pos();
				state.append(expected.clone(), *success);
				let node_type: NodeType<HasValidId> = NodeType::new_terminal_with_to_token(expected.clone(), *success);
				Node::new_with_pos(self.nullable, first_pos, last_pos, node_type)
			}

			NodeType::OneOf { lhs, rhs } => {
				let new_lhs: Node<HasValidId> = lhs.set_id(state);
				let new_rhs: Node<HasValidId> = rhs.set_id(state);
				let mut first_pos: Positions<FirstPos> = new_lhs.first_pos.clone();
				first_pos.union_first_pos(&new_rhs.first_pos);
				let mut last_pos: Positions<LastPos> = new_lhs.last_pos.clone();
				last_pos.union_last_pos(&new_rhs.last_pos);
				let node_type: NodeType<HasValidId> = NodeType::one_of(new_lhs, new_rhs);
				Node::new_with_pos(self.nullable, first_pos, last_pos, node_type)
			}

			NodeType::Repeat { repeat } => {
				let new_repeat: Node<HasValidId> = repeat.set_id(state);
				let first_pos: Positions<FirstPos> = new_repeat.first_pos.clone();
				let last_pos: Positions<LastPos> = new_repeat.last_pos.clone();
				let node_type: NodeType<HasValidId> = NodeType::repeat(new_repeat);
				Node::new_with_pos(self.nullable, first_pos, last_pos, node_type)
			}

			NodeType::Concat { lhs, rhs } => {
				let new_lhs: Node<HasValidId> = lhs.set_id(state);
				let new_rhs: Node<HasValidId> = rhs.set_id(state);
				let mut first_pos: Positions<FirstPos> = new_lhs.first_pos.clone();
				if new_lhs.nullable {
					first_pos.union_first_pos(&new_rhs.first_pos);
				}
				let mut last_pos: Positions<LastPos> = new_rhs.last_pos.clone();

				if new_rhs.nullable {
					last_pos.union_last_pos(&new_lhs.last_pos);
				}
				let node_type: NodeType<HasValidId> = NodeType::concat(new_lhs, new_rhs);
				Node::new_with_pos(self.nullable, first_pos, last_pos, node_type)
			}
		}
	}

	fn new<ID_FLAG>(
		nullable: bool,
		node_type: NodeType<ID_FLAG>,
	) -> Node<ID_FLAG>
	where
		ID_FLAG: Debug,
	{
		let first_pos: Positions<FirstPos> = positions::new_first_pos();
		let last_pos: Positions<LastPos> = positions::new_last_pos();
		Node {
			nullable,
			first_pos,
			last_pos,
			node_type,
			phantom: PhantomData,
		}
	}

	fn new_with_pos<ID_FLAG>(
		nullable: bool,
		first_pos: Positions<FirstPos>,
		last_pos: Positions<LastPos>,
		node_type: NodeType<ID_FLAG>,
	) -> Node<ID_FLAG>
	where
		ID_FLAG: Debug,
	{
		Node {
			nullable,
			first_pos,
			last_pos,
			node_type,
			phantom: PhantomData,
		}
	}

	fn with_node_type(
		self: &Self,
		node_type: NodeType<NoId>,
	) -> Self {
		Node {
			nullable: self.nullable,
			first_pos: self.first_pos.clone(),
			last_pos: self.last_pos.clone(),
			node_type,
			phantom: self.phantom,
		}
	}
}

impl Node<HasValidId> {
	pub(crate) fn get_start_ids(self: &Self) -> Positions<StartPos> {
		self.first_pos.to_start_pos()
	}
}

pub(crate) fn generate_follow_pos<ID_PROVIDER>(
	state: &mut ID_PROVIDER,
	node: &Node<HasValidId>,
) where
	ID_PROVIDER: IdProvider,
{
	match &node.node_type {
		NodeType::Empty => {
			// Nothing to do
		}

		NodeType::OneOf { lhs, rhs } => {
			generate_follow_pos(state, lhs);
			generate_follow_pos(state, rhs);
		}

		NodeType::Terminal { expected: _, success: _ } => {
			// Nothing to do
		}

		NodeType::Concat { lhs, rhs } => {
			generate_follow_pos(state, lhs);
			generate_follow_pos(state, rhs);
			lhs.last_pos.for_each(|id: Id| {
				update(state, id, &rhs.first_pos);
			});
		}

		NodeType::Repeat { repeat } => {
			generate_follow_pos(state, repeat);
			repeat.last_pos.for_each(|id: Id| {
				update(state, id, &repeat.first_pos);
			});
		}
	}
}

fn update<ID_PROVIDER>(
	state: &mut ID_PROVIDER,
	id: Id,
	first_pos: &Positions<FirstPos>,
) where
	ID_PROVIDER: IdProvider,
{
	let lexer_step: &mut LexerStep = state.get_lexer_step_mut(id);
	lexer_step.append_to_follow_pos(first_pos);
}

#[cfg(test)]
#[coverage(off)]
mod tests;
