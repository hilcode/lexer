use crate::hilcode::id_flag::NoId;
use crate::hilcode::node::Node;
use crate::hilcode::success::Success;
use ::imstr::ImString;
use ::std::fmt::Debug;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum NodeType<ID>
where
	ID: Debug,
{
	Empty,
	Terminal { expected: ImString, success: Option<Success> },
	Concat { lhs: Box<Node<ID>>, rhs: Box<Node<ID>> },
	OneOf { lhs: Box<Node<ID>>, rhs: Box<Node<ID>> },
	Repeat { repeat: Box<Node<ID>> },
}

impl NodeType<NoId> {
	pub(crate) fn terminal(expected: ImString) -> NodeType<NoId> {
		NodeType::Terminal { expected, success: None }
	}

	pub(crate) fn new_terminal_with_to_token<ID_FLAG>(
		expected: ImString,
		success: Option<Success>,
	) -> NodeType<ID_FLAG>
	where
		ID_FLAG: Debug,
	{
		NodeType::Terminal { expected, success }
	}

	pub(crate) fn one_of<ID_FLAG>(
		lhs: Node<ID_FLAG>,
		rhs: Node<ID_FLAG>,
	) -> NodeType<ID_FLAG>
	where
		ID_FLAG: Debug,
	{
		NodeType::OneOf {
			lhs: Box::new(lhs),
			rhs: Box::new(rhs),
		}
	}

	pub(crate) fn repeat<ID_FLAG>(node: Node<ID_FLAG>) -> NodeType<ID_FLAG>
	where
		ID_FLAG: Debug,
	{
		NodeType::Repeat { repeat: Box::new(node) }
	}

	pub(crate) fn concat<ID_FLAG>(
		lhs: Node<ID_FLAG>,
		rhs: Node<ID_FLAG>,
	) -> NodeType<ID_FLAG>
	where
		ID_FLAG: Debug,
	{
		NodeType::Concat {
			lhs: Box::new(lhs),
			rhs: Box::new(rhs),
		}
	}

	pub(crate) fn new_empty<ID_FLAG>() -> NodeType<ID_FLAG>
	where
		ID_FLAG: Debug,
	{
		NodeType::Empty
	}
}
