#[cfg(test)]
#[coverage(off)]
mod step {

	use crate::hilcode::Lexer;
	use crate::hilcode::dummy_token::DummyToken;
	use crate::hilcode::dummy_token::to_asterisk;
	use crate::hilcode::dummy_token::to_default;
	use crate::hilcode::fiber::Fiber;
	use crate::hilcode::test_data::TestData;
	use crate::hilcode::token_found::TokenFound;
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
		let lexer: Lexer<DummyToken> = test_data.lexer;
		let source: &ImString = &"*".into();
		let token_found: &mut Option<TokenFound<DummyToken>> = &mut test_data.token_found;
		let active_fibers: BTreeSet<Fiber> = test_data.active_fibers;
		let fibers: BTreeSet<Fiber> = lexer.step(source, token_found, active_fibers);
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
		let lexer: Lexer<DummyToken> = test_data.lexer;
		let source: &ImString = &"*".into();
		let token_found: &mut Option<TokenFound<DummyToken>> = &mut test_data.token_found;
		let active_fibers: BTreeSet<Fiber> = test_data.active_fibers;
		let fibers: BTreeSet<Fiber> = lexer.step(source, token_found, active_fibers);
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
		let error: DummyToken = DummyToken::Error("No valid token found".into());
		assert_eq!(token_it.next(), Some(error));
		assert_eq!(token_it.next(), None);
	}

	#[test]
	fn test_02() {
		let test_data = TestData::_builder()
			.lexer(Lexer::_builder().start_ids(&[0]).lexer_step_success(&[], "*", to_asterisk).build())
			.build();
		let mut token_it: LexerIt<DummyToken> = test_data.lexer.tokenize("*");
		let asterisk: DummyToken = DummyToken::Asterisk("*".into());
		assert_eq!(token_it.next(), Some(asterisk));
		assert_eq!(token_it.next(), None);
	}

	#[test]
	fn test_03() {
		let test_data = TestData::_builder()
			.lexer(Lexer::_builder().start_ids(&[0]).lexer_step_success(&[], "*", to_asterisk).build())
			.build();
		let mut token_it: LexerIt<DummyToken> = test_data.lexer.tokenize("**");
		let asterisk: DummyToken = DummyToken::Asterisk("*".into());
		assert_eq!(token_it.next(), Some(asterisk));
		let asterisk: DummyToken = DummyToken::Asterisk("*".into());
		assert_eq!(token_it.next(), Some(asterisk));
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
		let asterisk: DummyToken = DummyToken::Asterisk("*".into());
		assert_eq!(token_it.next(), Some(asterisk));
		let default: DummyToken = DummyToken::Default("?".into());
		assert_eq!(token_it.next(), Some(default));
		let asterisk: DummyToken = DummyToken::Asterisk("*".into());
		assert_eq!(token_it.next(), Some(asterisk));
		let default: DummyToken = DummyToken::Default("?".into());
		assert_eq!(token_it.next(), Some(default));
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
		let asterisk: DummyToken = DummyToken::Asterisk("ab".into());
		assert_eq!(token_it.next(), Some(asterisk));
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
			let asterisk: DummyToken = DummyToken::Asterisk("ABC".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("AB");
			let error: DummyToken = DummyToken::Error("No valid token found".into());
			assert_eq!(token_it.next(), Some(error));
			let error: DummyToken = DummyToken::Error("No valid token found".into());
			assert_eq!(token_it.next(), Some(error));
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
			let asterisk: DummyToken = DummyToken::Asterisk("A".into());
			assert_eq!(token_it.next(), Some(asterisk));
			let asterisk: DummyToken = DummyToken::Asterisk("B".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("C");
			let error: DummyToken = DummyToken::Error("No valid token found".into());
			assert_eq!(token_it.next(), Some(error));
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
			let asterisk: DummyToken = DummyToken::Asterisk("Aa".into());
			assert_eq!(token_it.next(), Some(asterisk));
			let asterisk: DummyToken = DummyToken::Asterisk("Ab".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("C");
			let error: DummyToken = DummyToken::Error("No valid token found".into());
			assert_eq!(token_it.next(), Some(error));
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
			let asterisk: DummyToken = DummyToken::Asterisk("ab".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("a");
			let error: DummyToken = DummyToken::Error("No valid token found".into());
			assert_eq!(token_it.next(), Some(error));
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
			let asterisk: DummyToken = DummyToken::Asterisk("A".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_1() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("AAbc");
			let asterisk: DummyToken = DummyToken::Asterisk("AAbc".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_2() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("AAbcAbc");
			let asterisk: DummyToken = DummyToken::Asterisk("AAbcAbc".into());
			assert_eq!(token_it.next(), Some(asterisk));
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
			let asterisk: DummyToken = DummyToken::Asterisk("Abc".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_2() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("AbcAbc");
			let asterisk: DummyToken = DummyToken::Asterisk("AbcAbc".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_3() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("AbcAbcAbc");
			let asterisk: DummyToken = DummyToken::Asterisk("AbcAbcAbc".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("A");
			let error: DummyToken = DummyToken::Error("No valid token found".into());
			assert_eq!(token_it.next(), Some(error));
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
			let asterisk: DummyToken = DummyToken::Asterisk("abcxyz".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_2() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("abcxyzxyz");
			let asterisk: DummyToken = DummyToken::Asterisk("abcxyzxyz".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_3() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("abcxyzxyzxyz");
			let asterisk: DummyToken = DummyToken::Asterisk("abcxyzxyzxyz".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("a");
			let error: DummyToken = DummyToken::Error("No valid token found".into());
			assert_eq!(token_it.next(), Some(error));
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
			let asterisk: DummyToken = DummyToken::Asterisk("ab".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_2() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("b");
			let asterisk: DummyToken = DummyToken::Asterisk("b".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("a");
			let error: DummyToken = DummyToken::Error("No valid token found".into());
			assert_eq!(token_it.next(), Some(error));
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
			let asterisk: DummyToken = DummyToken::Asterisk("xab".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn success_2() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("xb");
			let asterisk: DummyToken = DummyToken::Asterisk("xb".into());
			assert_eq!(token_it.next(), Some(asterisk));
			assert_eq!(token_it.next(), None);
		}

		#[test]
		fn failure() {
			let lexer: Lexer<DummyToken> = new_lexer();
			let mut token_it: LexerIt<DummyToken> = lexer.tokenize("x");
			let error: DummyToken = DummyToken::Error("No valid token found".into());
			assert_eq!(token_it.next(), Some(error));
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
