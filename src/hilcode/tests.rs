#[cfg(test)]
#[coverage(off)]
mod step {

	use crate::hilcode::Lexer;
	use crate::hilcode::dummy_token::to_asterisk;
	use crate::hilcode::dummy_token::to_default;
	use crate::hilcode::fiber::Fiber;
	use crate::hilcode::test_data::TestData;
	use ::imstr::ImString;
	use ::std::collections::BTreeSet;

	/*
	 * One step that checks for "*" and jumps to '0' should yield 1 fiber at offset '1' looking for "*".
	 */
	#[test]
	fn test_01() {
		let mut test_data = TestData::_builder()
			.lexer(Lexer::_builder().lexer_step_success(&[0], "*", to_asterisk).build())
			.active_fiber(Fiber::default())
			.build();
		let fibers: BTreeSet<Fiber> = test_data
			.lexer
			.step(&ImString::from("*"), &mut test_data.token_found, test_data.active_fibers);
		assert_eq!(fibers.len(), 1);
		assert!(fibers.contains(&Fiber::_builder().offset(1).build()));
	}

	/*
	 * One step that checks for "*" and jumps to '0' should yield 1 fiber at offset '1' looking for "*".
	 */
	#[test]
	fn test_02() {
		let mut test_data = TestData::_builder()
			.lexer(
				Lexer::_builder()
					.lexer_step_success(&[0, 1], "*", to_asterisk)
					.lexer_step_success(&[], "?", to_default)
					.build(),
			)
			.active_fiber(Fiber::default())
			.build();
		let fibers: BTreeSet<Fiber> = test_data
			.lexer
			.step(&ImString::from("*"), &mut test_data.token_found, test_data.active_fibers);
		assert_eq!(fibers.len(), 2);
		assert!(fibers.contains(&Fiber::_builder().position(0).offset(1).build()));
		assert!(fibers.contains(&Fiber::_builder().position(1).offset(1).build()));
	}
}

mod tokenize {

	use crate::hilcode::Lexer;
	use crate::hilcode::LexerIt;
	use crate::hilcode::dummy_token::DummyToken;
	use crate::hilcode::dummy_token::to_asterisk;
	use crate::hilcode::dummy_token::to_default;
	use crate::hilcode::test_data::TestData;

	#[test]
	fn test_01() {
		let test_data = TestData::_builder()
			.lexer(Lexer::_builder().start_ids(&[0]).lexer_step_success(&[], "*", to_asterisk).build())
			.build();
		let mut token_it: LexerIt<DummyToken> = test_data.lexer.tokenize("?");
		assert_eq!(token_it.next(), Some(DummyToken::Error("No valid token found".into())));
		assert_eq!(token_it.next(), None);
	}

	#[test]
	fn test_02() {
		let test_data = TestData::_builder()
			.lexer(Lexer::_builder().start_ids(&[0]).lexer_step_success(&[], "*", to_asterisk).build())
			.build();
		let mut token_it: LexerIt<DummyToken> = test_data.lexer.tokenize("*");
		assert_eq!(token_it.next(), Some(DummyToken::Asterisk("*".into())));
		assert_eq!(token_it.next(), None);
	}

	#[test]
	fn test_03() {
		let test_data = TestData::_builder()
			.lexer(Lexer::_builder().start_ids(&[0]).lexer_step_success(&[], "*", to_asterisk).build())
			.build();
		let mut token_it: LexerIt<DummyToken> = test_data.lexer.tokenize("**");
		assert_eq!(token_it.next(), Some(DummyToken::Asterisk("*".into())));
		assert_eq!(token_it.next(), Some(DummyToken::Asterisk("*".into())));
		assert_eq!(token_it.next(), None);
	}

	#[test]
	fn test_04() {
		let test_data = TestData::_builder()
			.lexer(
				Lexer::_builder()
					.start_ids(&[0, 1])
					.lexer_step_success(&[], "*", to_asterisk)
					.lexer_step_success(&[], "?", to_default)
					.build(),
			)
			.build();
		let mut token_it: LexerIt<DummyToken> = test_data.lexer.tokenize("*?*?");
		assert_eq!(token_it.next(), Some(DummyToken::Asterisk("*".into())));
		assert_eq!(token_it.next(), Some(DummyToken::Default("?".into())));
		assert_eq!(token_it.next(), Some(DummyToken::Asterisk("*".into())));
		assert_eq!(token_it.next(), Some(DummyToken::Default("?".into())));
		assert_eq!(token_it.next(), None);
	}

	#[test]
	fn test_05() {
		let test_data = TestData::_builder()
			.lexer(
				Lexer::_builder()
					.start_ids(&[0])
					.lexer_step(&[1], "a")
					.lexer_step_success(&[], "b", to_asterisk)
					.build(),
			)
			.build();
		let mut token_it: LexerIt<DummyToken> = test_data.lexer.tokenize("ab");
		assert_eq!(token_it.next(), Some(DummyToken::Asterisk("ab".into())));
		assert_eq!(token_it.next(), None);
	}
}

mod lexer {

	use crate::hilcode::Lexer;
	use crate::hilcode::dummy_token::DummyToken;
	use crate::hilcode::dummy_token::to_asterisk;
	use crate::hilcode::dummy_token::to_best;
	use crate::hilcode::dummy_token::to_default;
	use crate::hilcode::node;

	#[test]
	fn nullable_definition() {
		let result: Result<Lexer<DummyToken>, Vec<String>> = Lexer::builder()
			.new_token(node::repeat(node::text("Abc")), to_asterisk)
			.new_token(node::text("Abc"), to_asterisk)
			.new_token(node::repeat(node::text("Def")), to_default)
			.new_token(node::repeat(node::text("Xyz")), to_best)
			.build();
		let lexer_errors: Vec<String> = result.unwrap_err();
		assert_eq!(lexer_errors.len(), 3);
		assert_eq!(
			lexer_errors.as_ref(),
			vec![
				"Token #1 has a nullable definition".to_string(),
				"Token #3 has a nullable definition".to_string(),
				"Token #4 has a nullable definition".to_string()
			]
		)
	}

	mod text {

		use crate::hilcode::Lexer;
		use crate::hilcode::LexerIt;
		use crate::hilcode::dummy_token::DummyToken;
		use crate::hilcode::dummy_token::to_asterisk;
		use crate::hilcode::node;

		#[test]
		fn success() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("ABC");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("ABC".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("AB");
			assert_eq!(token_it.next(), Some(DummyToken::Error("No valid token found".into())));
			assert_eq!(token_it.next(), Some(DummyToken::Error("No valid token found".into())));
			assert_eq!(token_it.next(), None);
		}

		fn new_lexer() -> Lexer<DummyToken> {
			Lexer::builder().new_token(node::text("ABC"), to_asterisk).build().unwrap()
		}
	}

	mod one_of_1 {

		use crate::hilcode::Lexer;
		use crate::hilcode::LexerIt;
		use crate::hilcode::dummy_token::DummyToken;
		use crate::hilcode::dummy_token::to_asterisk;
		use crate::hilcode::node;

		#[test]
		fn success() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("AB");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("A".into())));
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("B".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("C");
			assert_eq!(token_it.next(), Some(DummyToken::Error("No valid token found".into())));
			assert_eq!(token_it.next(), None);
		}

		fn new_lexer() -> Lexer<DummyToken> {
			Lexer::builder()
				.new_token(node::one_of(node::text("A"), node::text("B")), to_asterisk)
				.build()
				.unwrap()
		}
	}

	mod one_of_2 {

		use crate::hilcode::Lexer;
		use crate::hilcode::LexerIt;
		use crate::hilcode::dummy_token::DummyToken;
		use crate::hilcode::dummy_token::to_asterisk;
		use crate::hilcode::node;

		#[test]
		fn success() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("AaAb");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("Aa".into())));
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("Ab".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("C");
			assert_eq!(token_it.next(), Some(DummyToken::Error("No valid token found".into())));
			assert_eq!(token_it.next(), None);
		}

		fn new_lexer() -> Lexer<DummyToken> {
			Lexer::builder()
				.new_token(node::text("A").one_of(node::text("a"), node::text("b")), to_asterisk)
				.build()
				.unwrap()
		}
	}

	mod concat {

		use crate::hilcode::Lexer;
		use crate::hilcode::LexerIt;
		use crate::hilcode::dummy_token::DummyToken;
		use crate::hilcode::dummy_token::to_asterisk;
		use crate::hilcode::node;

		#[test]
		fn success() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("ab");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("ab".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("a");
			assert_eq!(token_it.next(), Some(DummyToken::Error("No valid token found".into())));
			assert_eq!(token_it.next(), None);
		}

		fn new_lexer() -> Lexer<DummyToken> {
			Lexer::builder().new_token(node::text("a").text("b"), to_asterisk).build().unwrap()
		}
	}

	mod repeat {

		use crate::hilcode::Lexer;
		use crate::hilcode::LexerIt;
		use crate::hilcode::dummy_token::DummyToken;
		use crate::hilcode::dummy_token::to_asterisk;
		use crate::hilcode::node;

		#[test]
		fn success_0() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("A");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("A".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_1() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("AAbc");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("AAbc".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_2() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("AAbcAbc");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("AAbcAbc".into())));
			assert_eq!(token_it.next(), None);
		}

		fn new_lexer() -> Lexer<DummyToken> {
			Lexer::builder()
				.new_token(node::text("A").repeat(node::text("Abc")), to_asterisk)
				.build()
				.unwrap()
		}
	}

	mod repeat1_1 {

		use crate::hilcode::Lexer;
		use crate::hilcode::LexerIt;
		use crate::hilcode::dummy_token::DummyToken;
		use crate::hilcode::dummy_token::to_asterisk;
		use crate::hilcode::node;

		#[test]
		fn success_1() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("Abc");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("Abc".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_2() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("AbcAbc");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("AbcAbc".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_3() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("AbcAbcAbc");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("AbcAbcAbc".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("A");
			assert_eq!(token_it.next(), Some(DummyToken::Error("No valid token found".into())));
			assert_eq!(token_it.next(), None);
		}

		fn new_lexer() -> Lexer<DummyToken> {
			Lexer::builder()
				.new_token(node::repeat1(node::text("Abc")), to_asterisk)
				.build()
				.unwrap()
		}
	}

	mod repeat1_2 {

		use crate::hilcode::Lexer;
		use crate::hilcode::LexerIt;
		use crate::hilcode::dummy_token::DummyToken;
		use crate::hilcode::dummy_token::to_asterisk;
		use crate::hilcode::node;

		#[test]
		fn success_1() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("abcxyz");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("abcxyz".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_2() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("abcxyzxyz");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("abcxyzxyz".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_3() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("abcxyzxyzxyz");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("abcxyzxyzxyz".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("a");
			assert_eq!(token_it.next(), Some(DummyToken::Error("No valid token found".into())));
			assert_eq!(token_it.next(), None);
		}

		fn new_lexer() -> Lexer<DummyToken> {
			Lexer::builder()
				.new_token(node::text("abc").repeat1(node::text("xyz")), to_asterisk)
				.build()
				.unwrap()
		}
	}

	mod optionally_1 {

		use crate::hilcode::Lexer;
		use crate::hilcode::LexerIt;
		use crate::hilcode::dummy_token::DummyToken;
		use crate::hilcode::dummy_token::to_asterisk;
		use crate::hilcode::node;

		#[test]
		fn success_1() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("ab");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("ab".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_2() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("b");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("b".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("a");
			assert_eq!(token_it.next(), Some(DummyToken::Error("No valid token found".into())));
			assert_eq!(token_it.next(), None);
		}

		fn new_lexer() -> Lexer<DummyToken> {
			Lexer::builder()
				.new_token(node::optionally(node::text("a")).text("b"), to_asterisk)
				.build()
				.unwrap()
		}
	}

	mod optionally_2 {

		use crate::hilcode::Lexer;
		use crate::hilcode::LexerIt;
		use crate::hilcode::dummy_token::DummyToken;
		use crate::hilcode::dummy_token::to_asterisk;
		use crate::hilcode::node;

		#[test]
		fn success_1() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("xab");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("xab".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_2() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("xb");
			assert_eq!(token_it.next(), Some(DummyToken::Asterisk("xb".into())));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("x");
			assert_eq!(token_it.next(), Some(DummyToken::Error("No valid token found".into())));
			assert_eq!(token_it.next(), None);
		}

		fn new_lexer() -> Lexer<DummyToken> {
			Lexer::builder()
				.new_token(node::text("x").optionally(node::text("a")).text("b"), to_asterisk)
				.build()
				.unwrap()
		}
	}
}
