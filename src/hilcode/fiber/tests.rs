use crate::hilcode::dummy_token::DummyToken;
use crate::hilcode::fiber::Fiber;
use ::proptest::prelude::Just;
use ::proptest::prelude::Strategy;
use ::proptest::prop_oneof;
use ::proptest::proptest;
use ::std::cmp::Ordering;

fn gen_position() -> impl Strategy<Value = usize> {
	prop_oneof![Just(0), 1..9usize, 10..100usize, 100..1000usize,]
}

fn gen_offset() -> impl Strategy<Value = usize> {
	prop_oneof![Just(0), 1..9usize, 10..100usize, 100..1000usize,]
}

fn gen_fiber() -> impl Strategy<Value = Fiber> {
	Strategy::prop_map((gen_position(), gen_offset()), move |(position, offset)| -> Fiber {
		return Fiber::new(position, offset);
	})
}

proptest! {
	#[test]
	fn check_eq(lhs in gen_fiber(), rhs in gen_fiber()) {
		if lhs == rhs {
			assert_eq!(rhs, lhs);
		} else {
			assert_ne!(rhs, lhs);
		}
	}

	#[test]
	fn check_ord(lhs in gen_fiber(), rhs in gen_fiber()) {
		match lhs.cmp(&rhs) {
			Ordering::Less => {
				assert!(rhs > lhs);
			}

			Ordering::Equal => {
				assert!(rhs == lhs && lhs == rhs);
			}

			Ordering::Greater => {
				assert!(rhs < lhs);
			}
		}
	}
}

mod run {

	use crate::hilcode::Lexer;
	use crate::hilcode::TokenFound;
	use crate::hilcode::dummy_token::to_asterisk;
	use crate::hilcode::fiber::Fiber;
	use crate::hilcode::fiber::tests::DummyToken;
	use crate::hilcode::test_data::TestData;
	use ::imstr::ImString;
	use ::std::collections::BTreeSet;

	/*
	 * The LexerStep matches and has a token. No token has been found yet. The matching token should be the new token found.
	 */
	#[test]
	fn test_01() {
		let mut test_data: TestData<DummyToken> = TestData::_builder()
			.lexer(Lexer::_builder().lexer_step_success(&[], "*", to_asterisk).build())
			.build();
		let fiber: Fiber = test_data.fiber;
		let token_found: &mut Option<TokenFound<DummyToken>> = &mut test_data.token_found;
		let active_fibers: &mut BTreeSet<Fiber> = &mut test_data.active_fibers;
		let source: &ImString = &"*".into();
		let lexer: &Lexer<DummyToken> = &test_data.lexer;
		fiber.run(token_found, active_fibers, source, lexer);
		assert_eq!(test_data.active_fibers.len(), 0);
		let token_found: TokenFound<DummyToken> = TokenFound::new(DummyToken::Asterisk("*".into()), 1);
		assert_eq!(test_data.token_found, Some(token_found));
	}

	/*
	 * The LexerStep matches and has a token. A superior token has already been found. The matching token should not replace it.
	 */
	#[test]
	fn test_02() {
		let mut test_data: TestData<DummyToken> = TestData::_builder()
			.lexer(Lexer::_builder().lexer_step_success(&[], "*", to_asterisk).build())
			.token_found(TokenFound::new(DummyToken::Best("...".into()), 0))
			.build();
		let fiber: Fiber = test_data.fiber;
		let token_found: &mut Option<TokenFound<DummyToken>> = &mut test_data.token_found;
		let active_fibers: &mut BTreeSet<Fiber> = &mut test_data.active_fibers;
		let source: &ImString = &"*".into();
		let lexer: &Lexer<DummyToken> = &test_data.lexer;
		fiber.run(token_found, active_fibers, source, lexer);
		assert_eq!(test_data.active_fibers.len(), 0);
		assert_eq!(test_data.token_found, Some(TokenFound::new(DummyToken::Best("...".into()), 0)));
	}

	/*
	 * The LexerStep matches and has a token. No token has been found yet. The matching token should be the new token found.
	 *
	 * The token in the fiber should be ignored.
	 */
	#[test]
	fn test_03() {
		let mut test_data: TestData<DummyToken> = TestData::_builder()
			.lexer(Lexer::_builder().lexer_step_success(&[], "*", to_asterisk).build())
			.fiber(Fiber::default())
			.build();
		let fiber: Fiber = test_data.fiber;
		let token_found: &mut Option<TokenFound<DummyToken>> = &mut test_data.token_found;
		let active_fibers: &mut BTreeSet<Fiber> = &mut test_data.active_fibers;
		let source: &ImString = &"*".into();
		let lexer: &Lexer<DummyToken> = &test_data.lexer;
		fiber.run(token_found, active_fibers, source, lexer);
		assert_eq!(test_data.active_fibers.len(), 0);
		assert_eq!(test_data.token_found, Some(TokenFound::new(DummyToken::Asterisk("*".into()), 1)));
	}

	/*
	 * If the LexerStep does not match than the token found so far should not change.
	 */
	#[test]
	fn test_04() {
		let mut test_data: TestData<DummyToken> = TestData::_builder()
			.lexer(Lexer::_builder().lexer_step_success(&[], "*", to_asterisk).build())
			.fiber(Fiber::default())
			.token_found(TokenFound::new(DummyToken::default(), 0))
			.build();
		let fiber: Fiber = test_data.fiber;
		let token_found: &mut Option<TokenFound<DummyToken>> = &mut test_data.token_found;
		let active_fibers: &mut BTreeSet<Fiber> = &mut test_data.active_fibers;
		let source: &ImString = &"...".into();
		let lexer: &Lexer<DummyToken> = &test_data.lexer;
		fiber.run(token_found, active_fibers, source, lexer);
		assert_eq!(test_data.active_fibers.len(), 0);
		assert_eq!(test_data.token_found, Some(TokenFound::new(DummyToken::default(), 0)));
	}

	/*
	 * The LexerStep matches and has a token. No token has been found yet. The matching token should be the new token found.
	 *
	 * There are 2 position ids in "next" so we should have 2 active fibers afterwards.
	 */
	#[test]
	fn test_05() {
		let mut test_data: TestData<DummyToken> = TestData::_builder()
			.lexer(Lexer::_builder().lexer_step_success(&[0, 5], "*", to_asterisk).build())
			.fiber(Fiber::default())
			.build();
		let fiber: Fiber = test_data.fiber;
		let token_found: &mut Option<TokenFound<DummyToken>> = &mut test_data.token_found;
		let active_fibers: &mut BTreeSet<Fiber> = &mut test_data.active_fibers;
		let source: &ImString = &"**".into();
		let lexer: &Lexer<DummyToken> = &test_data.lexer;
		fiber.run(token_found, active_fibers, source, lexer);
		assert_eq!(test_data.active_fibers.len(), 2);
		assert!(test_data.active_fibers.contains(&Fiber::_builder().offset(1).build()));
		assert!(test_data.active_fibers.contains(&Fiber::_builder().position(5).offset(1).build()));
		assert_eq!(test_data.token_found, Some(TokenFound::new(DummyToken::Asterisk("*".into()), 1)));
	}

	/*
	 * The LexerStep matches and has a token. An inferior token has been found. The matching token should be the new token found.
	 *
	 * There is 1 position id in "next" so we should have 1 active fiber afterwards.
	 */
	#[test]
	fn test_06() {
		let mut test_data: TestData<DummyToken> = TestData::_builder()
			.lexer(Lexer::_builder().lexer_step_success(&[0], "*", to_asterisk).build())
			.fiber(Fiber::default())
			.token_found(TokenFound::new(DummyToken::default(), 0))
			.build();
		let fiber: Fiber = test_data.fiber;
		let token_found: &mut Option<TokenFound<DummyToken>> = &mut test_data.token_found;
		let active_fibers: &mut BTreeSet<Fiber> = &mut test_data.active_fibers;
		let source: &ImString = &"**".into();
		let lexer: &Lexer<DummyToken> = &test_data.lexer;
		fiber.run(token_found, active_fibers, source, lexer);
		assert_eq!(test_data.active_fibers.len(), 1);
		let expected_fiber: Fiber = Fiber::_builder().offset(1).build();
		assert!(test_data.active_fibers.contains(&expected_fiber));
		assert_eq!(test_data.token_found, Some(TokenFound::new(DummyToken::Asterisk("*".into()), 1)));
	}

	/*
	 * The LexerStep matches and has a token. A superior token has been found. The matching token should not replace the current token found.
	 *
	 * There are 3 position ids in "next" so we should have 3 active fibers afterwards.
	 */
	#[test]
	fn test_07() {
		let mut test_data: TestData<DummyToken> = TestData::_builder()
			.lexer(Lexer::_builder().lexer_step_success(&[0, 100, 20], "*", to_asterisk).build())
			.fiber(Fiber::default())
			.token_found(TokenFound::new(DummyToken::Best("...".into()), 0))
			.build();
		let fiber: Fiber = test_data.fiber;
		let token_found: &mut Option<TokenFound<DummyToken>> = &mut test_data.token_found;
		let active_fibers: &mut BTreeSet<Fiber> = &mut test_data.active_fibers;
		let source: &ImString = &"**".into();
		let lexer: &Lexer<DummyToken> = &test_data.lexer;
		fiber.run(token_found, active_fibers, source, lexer);
		assert_eq!(test_data.active_fibers.len(), 3);
		let expected_fiber: Fiber = Fiber::_builder().offset(1).build();
		assert!(test_data.active_fibers.contains(&expected_fiber));
		let expected_fiber: Fiber = Fiber::_builder().position(100).offset(1).build();
		assert!(test_data.active_fibers.contains(&expected_fiber));
		let expected_fiber: Fiber = Fiber::_builder().position(20).offset(1).build();
		assert!(test_data.active_fibers.contains(&expected_fiber));
		assert_eq!(test_data.token_found, Some(TokenFound::new(DummyToken::Best("...".into()), 0)));
	}

	/*
	 * The LexerStep matches and has no token. No token has been found yet. This should be a total dud.
	 */
	#[test]
	fn test_08() {
		let mut test_data: TestData<DummyToken> = TestData::_builder().lexer(Lexer::_builder().lexer_step(&[], "*").build()).build();
		let fiber: Fiber = test_data.fiber;
		let token_found: &mut Option<TokenFound<DummyToken>> = &mut test_data.token_found;
		let active_fibers: &mut BTreeSet<Fiber> = &mut test_data.active_fibers;
		let source: &ImString = &"*".into();
		let lexer: &Lexer<DummyToken> = &test_data.lexer;
		fiber.run(token_found, active_fibers, source, lexer);
		assert_eq!(test_data.active_fibers.len(), 0);
		assert_eq!(test_data.token_found, None);
	}
}
