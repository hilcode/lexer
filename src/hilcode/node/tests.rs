mod text {

	use crate::hilcode::id_flag::HasValidId;
	use crate::hilcode::id_flag::NoId;
	use crate::hilcode::id_provider::IdProviderState;
	use crate::hilcode::node;
	use crate::hilcode::node::Node;
	use crate::hilcode::node_type::NodeType;
	use crate::hilcode::positions::FirstPos;
	use crate::hilcode::positions::LastPos;
	use crate::hilcode::positions::Positions;
	use crate::hilcode::positions::StartPos;
	use crate::hilcode::token_id::TokenId;

	#[test]
	fn without_token() {
		let root: Node<NoId> = node::text("*");
		let mut state: IdProviderState = IdProviderState::default();
		let root: Node<HasValidId> = root.set_id(&mut state);
		assert_eq!(root.nullable, false);
		assert_eq!(root.first_pos, Positions::<FirstPos>::new(&[0]));
		assert_eq!(root.last_pos, Positions::<LastPos>::new(&[0]));
		assert_eq!(root.get_start_ids(), Positions::<StartPos>::new(&[0]));
		assert_eq!(
			root.node_type,
			NodeType::Terminal {
				expected: "*".into(),
				success: None
			}
		);
	}

	#[test]
	fn with_token() {
		let root: Node<NoId> = node::text("*");
		let mut state: IdProviderState = IdProviderState::default();
		let success: TokenId = TokenId::new(100);
		let root: Node<NoId> = root.set_success_token_id(success);
		let root: Node<HasValidId> = root.set_id(&mut state);
		assert_eq!(root.nullable, false);
		assert_eq!(root.first_pos, Positions::<FirstPos>::new(&[0]));
		assert_eq!(root.last_pos, Positions::<LastPos>::new(&[0]));
		assert_eq!(root.get_start_ids(), Positions::<StartPos>::new(&[0]));
		assert_eq!(
			root.node_type,
			NodeType::Terminal {
				expected: "*".into(),
				success: Some(success),
			}
		);
	}
}

mod one_of {

	use crate::hilcode::id_flag::HasValidId;
	use crate::hilcode::id_flag::NoId;
	use crate::hilcode::id_provider::IdProviderState;
	use crate::hilcode::node;
	use crate::hilcode::node::Node;
	use crate::hilcode::node_type::NodeType;
	use crate::hilcode::positions::FirstPos;
	use crate::hilcode::positions::LastPos;
	use crate::hilcode::positions::Positions;
	use crate::hilcode::positions::StartPos;
	use crate::hilcode::token_id::TokenId;

	mod nullable {

		use crate::hilcode::id_flag::HasValidId;
		use crate::hilcode::id_flag::NoId;
		use crate::hilcode::id_provider::IdProviderState;
		use crate::hilcode::node;
		use crate::hilcode::node::Node;

		#[test]
		fn yes_yes() {
			let root: Node<NoId> = node::one_of(node::empty(), node::empty());
			let mut state: IdProviderState = IdProviderState::default();
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.nullable, true);
		}

		#[test]
		fn yes_no() {
			let root: Node<NoId> = node::one_of(node::empty(), node::text("B"));
			let mut state: IdProviderState = IdProviderState::default();
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.nullable, true);
		}

		#[test]
		fn no_yes() {
			let root: Node<NoId> = node::one_of(node::text("A"), node::empty());
			let mut state: IdProviderState = IdProviderState::default();
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.nullable, true);
		}

		#[test]
		fn no_no() {
			let root: Node<NoId> = node::one_of(node::text("A"), node::text("B"));
			let mut state: IdProviderState = IdProviderState::default();
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.nullable, false);
		}
	}

	#[test]
	fn without_token() {
		let lhs: Node<NoId> = node::text("A");
		let rhs: Node<NoId> = node::text("B");
		let root: Node<NoId> = node::one_of(lhs, rhs);
		let mut state: IdProviderState = IdProviderState::default();
		let root: Node<HasValidId> = root.set_id(&mut state);
		assert_eq!(root.first_pos, Positions::<FirstPos>::new(&[0, 1]));
		assert_eq!(root.last_pos, Positions::<LastPos>::new(&[0, 1]));
		assert_eq!(root.get_start_ids(), Positions::<StartPos>::new(&[0, 1]));
		match root.node_type {
			NodeType::OneOf { lhs, rhs } => {
				assert_eq!(
					lhs.node_type,
					NodeType::Terminal {
						expected: "A".into(),
						success: None
					}
				);
				assert_eq!(
					rhs.node_type,
					NodeType::Terminal {
						expected: "B".into(),
						success: None
					}
				);
			}

			_ => {
				assert!(false, "Incorrect NodeType");
			}
		}
	}

	#[test]
	fn with_token() {
		let lhs: Node<NoId> = node::text("A");
		let rhs: Node<NoId> = node::text("B");
		let root: Node<NoId> = node::one_of(lhs, rhs);
		let mut state: IdProviderState = IdProviderState::default();
		let success: TokenId = TokenId::new(100);
		let root: Node<NoId> = root.set_success_token_id(success);
		let root: Node<HasValidId> = root.set_id(&mut state);
		assert_eq!(root.first_pos, Positions::<FirstPos>::new(&[0, 1]));
		assert_eq!(root.last_pos, Positions::<LastPos>::new(&[0, 1]));
		assert_eq!(root.get_start_ids(), Positions::<StartPos>::new(&[0, 1]));
		match root.node_type {
			NodeType::OneOf { lhs, rhs } => {
				assert_eq!(
					lhs.node_type,
					NodeType::Terminal {
						expected: "A".into(),
						success: Some(success)
					}
				);
				assert_eq!(
					rhs.node_type,
					NodeType::Terminal {
						expected: "B".into(),
						success: Some(success)
					}
				);
			}

			_ => {
				assert!(false, "Incorrect NodeType");
			}
		}
	}
}

mod concat {

	mod nullable {

		use crate::hilcode::id_flag::HasValidId;
		use crate::hilcode::id_flag::NoId;
		use crate::hilcode::id_provider::IdProviderState;
		use crate::hilcode::node;
		use crate::hilcode::node::Node;

		#[test]
		fn yes_yes() {
			let root: Node<NoId> = node::concat(node::empty(), node::empty());
			let mut state: IdProviderState = IdProviderState::default();
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.nullable, true);
		}

		#[test]
		fn yes_no() {
			let root: Node<NoId> = node::concat(node::empty(), node::text("B"));
			let mut state: IdProviderState = IdProviderState::default();
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.nullable, false);
		}

		#[test]
		fn no_yes() {
			let root: Node<NoId> = node::concat(node::text("A"), node::empty());
			let mut state: IdProviderState = IdProviderState::default();
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.nullable, false);
		}

		#[test]
		fn no_no() {
			let root: Node<NoId> = node::concat(node::text("A"), node::text("B"));
			let mut state: IdProviderState = IdProviderState::default();
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.nullable, false);
		}
	}

	mod without_token {

		use crate::hilcode::id_flag::HasValidId;
		use crate::hilcode::id_flag::NoId;
		use crate::hilcode::id_provider::IdProviderState;
		use crate::hilcode::node;
		use crate::hilcode::node::Node;
		use crate::hilcode::node_type::NodeType;
		use crate::hilcode::positions::FirstPos;
		use crate::hilcode::positions::LastPos;
		use crate::hilcode::positions::Positions;
		use crate::hilcode::positions::StartPos;

		#[test]
		fn yes_no() {
			let lhs: Node<NoId> = node::empty();
			let rhs: Node<NoId> = node::text("B");
			let root: Node<NoId> = node::concat(lhs, rhs);
			let mut state: IdProviderState = IdProviderState::default();
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.first_pos, Positions::<FirstPos>::new(&[0]));
			assert_eq!(root.last_pos, Positions::<LastPos>::new(&[0]));
			assert_eq!(root.get_start_ids(), Positions::<StartPos>::new(&[0]));
			match root.node_type {
				NodeType::Concat { lhs, rhs } => {
					assert_eq!(lhs.node_type, NodeType::Empty);
					assert_eq!(
						rhs.node_type,
						NodeType::Terminal {
							expected: "B".into(),
							success: None
						}
					);
				}

				_ => {
					assert!(false, "Incorrect NodeType");
				}
			}
		}

		#[test]
		fn no_yes() {
			let lhs: Node<NoId> = node::text("A");
			let rhs: Node<NoId> = node::empty();
			let root: Node<NoId> = node::concat(lhs, rhs);
			let mut state: IdProviderState = IdProviderState::default();
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.first_pos, Positions::<FirstPos>::new(&[0]));
			assert_eq!(root.last_pos, Positions::<LastPos>::new(&[0]));
			assert_eq!(root.get_start_ids(), Positions::<StartPos>::new(&[0]));
			match root.node_type {
				NodeType::Concat { lhs, rhs } => {
					assert_eq!(
						lhs.node_type,
						NodeType::Terminal {
							expected: "A".into(),
							success: None
						}
					);
					assert_eq!(rhs.node_type, NodeType::Empty);
				}

				_ => {
					assert!(false, "Incorrect NodeType");
				}
			}
		}

		#[test]
		fn no_no() {
			let lhs: Node<NoId> = node::text("A");
			let rhs: Node<NoId> = node::text("B");
			let root: Node<NoId> = node::concat(lhs, rhs);
			let mut state: IdProviderState = IdProviderState::default();
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.first_pos, Positions::<FirstPos>::new(&[0]));
			assert_eq!(root.last_pos, Positions::<LastPos>::new(&[1]));
			assert_eq!(root.get_start_ids(), Positions::<StartPos>::new(&[0]));
			match root.node_type {
				NodeType::Concat { lhs, rhs } => {
					assert_eq!(
						lhs.node_type,
						NodeType::Terminal {
							expected: "A".into(),
							success: None
						}
					);
					assert_eq!(
						rhs.node_type,
						NodeType::Terminal {
							expected: "B".into(),
							success: None
						}
					);
				}

				_ => {
					assert!(false, "Incorrect NodeType");
				}
			}
		}
	}

	mod with_token {

		use crate::hilcode::id_flag::HasValidId;
		use crate::hilcode::id_flag::NoId;
		use crate::hilcode::id_provider::IdProviderState;
		use crate::hilcode::node;
		use crate::hilcode::node::Node;
		use crate::hilcode::node_type::NodeType;
		use crate::hilcode::positions::FirstPos;
		use crate::hilcode::positions::LastPos;
		use crate::hilcode::positions::Positions;
		use crate::hilcode::positions::StartPos;
		use crate::hilcode::token_id::TokenId;

		#[test]
		fn yes_no() {
			let lhs: Node<NoId> = node::empty();
			let rhs: Node<NoId> = node::text("B");
			let root: Node<NoId> = node::concat(lhs, rhs);
			let mut state: IdProviderState = IdProviderState::default();
			let success: TokenId = TokenId::new(100);
			let root: Node<NoId> = root.set_success_token_id(success);
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.first_pos, Positions::<FirstPos>::new(&[0]));
			assert_eq!(root.last_pos, Positions::<LastPos>::new(&[0]));
			assert_eq!(root.get_start_ids(), Positions::<StartPos>::new(&[0]));
			match root.node_type {
				NodeType::Concat { lhs, rhs } => {
					assert_eq!(lhs.node_type, NodeType::Empty);
					assert_eq!(
						rhs.node_type,
						NodeType::Terminal {
							expected: "B".into(),
							success: Some(success),
						}
					);
				}

				_ => {
					assert!(false, "Incorrect NodeType");
				}
			}
		}

		#[test]
		fn no_yes() {
			let lhs: Node<NoId> = node::text("A");
			let rhs: Node<NoId> = node::empty();
			let root: Node<NoId> = node::concat(lhs, rhs);
			let mut state: IdProviderState = IdProviderState::default();
			let success: TokenId = TokenId::new(100);
			let root: Node<NoId> = root.set_success_token_id(success);
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.first_pos, Positions::<FirstPos>::new(&[0]));
			assert_eq!(root.last_pos, Positions::<LastPos>::new(&[0]));
			assert_eq!(root.get_start_ids(), Positions::<StartPos>::new(&[0]));
			match root.node_type {
				NodeType::Concat { lhs, rhs } => {
					assert_eq!(
						lhs.node_type,
						NodeType::Terminal {
							expected: "A".into(),
							success: Some(success),
						}
					);
					assert_eq!(rhs.node_type, NodeType::Empty);
				}

				_ => {
					assert!(false, "Incorrect NodeType");
				}
			}
		}

		#[test]
		fn no_no() {
			let lhs: Node<NoId> = node::text("A");
			let rhs: Node<NoId> = node::text("B");
			let root: Node<NoId> = node::concat(lhs, rhs);
			let mut state: IdProviderState = IdProviderState::default();
			let success: TokenId = TokenId::new(100);
			let root: Node<NoId> = root.set_success_token_id(success);
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.first_pos, Positions::<FirstPos>::new(&[0]));
			assert_eq!(root.last_pos, Positions::<LastPos>::new(&[1]));
			assert_eq!(root.get_start_ids(), Positions::<StartPos>::new(&[0]));
			match root.node_type {
				NodeType::Concat { lhs, rhs } => {
					assert_eq!(
						lhs.node_type,
						NodeType::Terminal {
							expected: "A".into(),
							success: None
						}
					);
					assert_eq!(
						rhs.node_type,
						NodeType::Terminal {
							expected: "B".into(),
							success: Some(success),
						}
					);
				}

				_ => {
					assert!(false, "Incorrect NodeType");
				}
			}
		}
	}
}

mod repeat {

	use crate::hilcode::id_flag::HasValidId;
	use crate::hilcode::id_flag::NoId;
	use crate::hilcode::id_provider::IdProviderState;
	use crate::hilcode::node;
	use crate::hilcode::node::Node;
	use crate::hilcode::node_type::NodeType;
	use crate::hilcode::positions::FirstPos;
	use crate::hilcode::positions::LastPos;
	use crate::hilcode::positions::Positions;
	use crate::hilcode::positions::StartPos;
	use crate::hilcode::token_id::TokenId;

	mod nullable {

		use crate::hilcode::id_flag::HasValidId;
		use crate::hilcode::id_flag::NoId;
		use crate::hilcode::id_provider::IdProviderState;
		use crate::hilcode::node;
		use crate::hilcode::node::Node;

		#[test]
		fn yes() {
			let root: Node<NoId> = node::repeat(node::empty());
			let mut state: IdProviderState = IdProviderState::default();
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.nullable, true);
		}

		#[test]
		fn no() {
			let root: Node<NoId> = node::repeat(node::text("A"));
			let mut state: IdProviderState = IdProviderState::default();
			let root: Node<HasValidId> = root.set_id(&mut state);
			assert_eq!(root.nullable, true);
		}
	}

	#[test]
	fn without_token() {
		let repeat: Node<NoId> = node::text("A");
		let root: Node<NoId> = node::repeat(repeat);
		let mut state: IdProviderState = IdProviderState::default();
		let root: Node<HasValidId> = root.set_id(&mut state);
		assert_eq!(root.first_pos, Positions::<FirstPos>::new(&[0]));
		assert_eq!(root.last_pos, Positions::<LastPos>::new(&[0]));
		assert_eq!(root.get_start_ids(), Positions::<StartPos>::new(&[0]));
		match root.node_type {
			NodeType::Repeat { repeat } => {
				assert_eq!(
					repeat.node_type,
					NodeType::Terminal {
						expected: "A".into(),
						success: None
					}
				);
			}

			_ => {
				assert!(false, "Incorrect NodeType");
			}
		}
	}

	#[test]
	fn with_token() {
		let repeat: Node<NoId> = node::text("A");
		let root: Node<NoId> = node::repeat(repeat);
		let mut state: IdProviderState = IdProviderState::default();
		let success: TokenId = TokenId::new(100);
		let root: Node<NoId> = root.set_success_token_id(success);
		let root: Node<HasValidId> = root.set_id(&mut state);
		assert_eq!(root.first_pos, Positions::<FirstPos>::new(&[0]));
		assert_eq!(root.last_pos, Positions::<LastPos>::new(&[0]));
		assert_eq!(root.get_start_ids(), Positions::<StartPos>::new(&[0]));
		match root.node_type {
			NodeType::Repeat { repeat } => {
				assert_eq!(
					repeat.node_type,
					NodeType::Terminal {
						expected: "A".into(),
						success: Some(success),
					}
				);
			}

			_ => {
				assert!(false, "Incorrect NodeType");
			}
		}
	}
}
