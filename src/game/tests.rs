use super::{ruleset::Rule, *};

#[test]
fn rule_1_test() {
    let mut test_game = PasswordGame::new();
    test_game.set_rule(Rule::FiveChars);

    assert!(test_game.validate_password("abcde"));
    assert!(test_game.validate_password("abcdefghij"));
    assert!(!test_game.validate_password("abcd"));
}

#[test]
fn rule_2_test() {
    let mut test_game = PasswordGame::new();
    test_game.set_rule(Rule::IncludeNum);

    assert!(test_game.validate_password("5"));
    assert!(test_game.validate_password("foo9"));
    assert!(test_game.validate_password("30\u{0BF0}foo"));
    assert!(!test_game.validate_password("foo"));
    assert!(!test_game.validate_password("\u{0BF0}\u{2169}\u{516D}"));
}

#[test]
fn rule_3_test() {
    let mut test_game = PasswordGame::new();
    test_game.set_rule(Rule::IncludeUpper);

    assert!(test_game.validate_password("A"));
    assert!(test_game.validate_password("abcdeFgHiJ"));
    assert!(!test_game.validate_password("a"));
    assert!(!test_game.validate_password("abcdefghij"));
}

#[test]
fn rule_4_test() {
    let mut test_game = PasswordGame::new();
    test_game.set_rule(Rule::IncludeSpecial);

    assert!(test_game.validate_password("@"));
    assert!(test_game.validate_password("a.bcd-ef@"));
    assert!(test_game.validate_password("ççç"));
    assert!(test_game.validate_password("\u{0436}"));
    assert!(!test_game.validate_password("abc"));
    assert!(!test_game.validate_password("3000AAAAF"));
}

#[test]
fn rule_5_test() {
    let mut test_game = PasswordGame::new();
    test_game.set_rule(Rule::AddTo25);

    assert!(test_game.validate_password("997"));
    assert!(!test_game.validate_password("999999999999"));
    assert!(test_game.validate_password("a5bcd5.-5@@5AA5"));
    assert!(!test_game.validate_password("996"));
    assert!(!test_game.validate_password("5bd5.@5AA5"));
}

#[test]
fn rule_6_test() {
    let mut test_game = PasswordGame::new();
    test_game.set_rule(Rule::IncludeMonth);

    assert!(test_game.validate_password("january"));
    assert!(test_game.validate_password("00--dEcEmbER,."));
    assert!(!test_game.validate_password("januaaaaary"));
    assert!(!test_game.validate_password("apil"));
}

#[test]
fn rule_7_test() {
    let mut test_game = PasswordGame::new();
    test_game.set_rule(Rule::IncludeRoman);

    assert!(test_game.validate_password("V"));
    assert!(test_game.validate_password("MILLION"));
    assert!(test_game.validate_password("abcdefghIj"));
    assert!(!test_game.validate_password("million"));
}

// #[test]
// fn rule_8_test() {
//     let mut test_game = PasswordGame::new();
//     todo!()
// }

// #[test]
// fn rule_9_test() {
//     let mut test_game = PasswordGame::new();
//     todo!()
// }

// #[test]
// fn rule_10_test() {
//     let mut test_game = PasswordGame::new();
//     todo!()
// }

// #[test]
// fn rule_11_test() {
//     let mut test_game = PasswordGame::new();
//     todo!()
// }

// #[test]
// fn rule_12_test() {
//     let mut test_game = PasswordGame::new();
//     todo!()
// }

// #[test]
// fn rule_13_test() {
//     let mut test_game = PasswordGame::new();
//     todo!()
// }

// #[test]
// fn rule_14_test() {
//     let mut test_game = PasswordGame::new();
//     todo!()
// }

#[test]
fn rule_15_test() {
    let mut test_game = PasswordGame::new();
    test_game.set_rule(Rule::IncludeLeapYear);

    assert!(test_game.validate_password("abcd0ef"));
    assert!(test_game.validate_password("20AAAAA"));
    assert!(test_game.validate_password("2123124"));
    assert!(!test_game.validate_password("a-~30e..ç"));
    assert!(!test_game.validate_password("2200"));
}

// #[test]
// fn rule_16_test() {
//     let mut test_game = PasswordGame::new();
//     todo!()
// }

// #[test]
// fn rule_17_test() {
//     let mut test_game = PasswordGame::new();
//     todo!()
// }

// #[test]
// fn rule_18_test() {
//     let mut test_game = PasswordGame::new();
//     todo!()
// }

#[test]
fn rule_21_test() {
    let mut test_game = PasswordGame::new();
    test_game.set_rule(Rule::StrongEnough);

    assert!(test_game.validate_password("\u{1f3cb}\u{1f3cb}\u{1f3cb}"));
    assert!(test_game.validate_password("\u{1f3cb}01213.as\u{1f3cb}dsakma\u{1f3cb}"));
    assert!(!test_game.validate_password("\u{1f3cb}\u{1f3cb}"));
}

#[test]
fn rule_22_test() {
    let mut test_game = PasswordGame::new();
    test_game.set_rule(Rule::IncludeAffirmations);

    assert!(test_game.validate_password("i am loved"));
    assert!(test_game.validate_password("a1231.@i am worthyll.;"));
    assert!(test_game.validate_password("iamenough"));
    assert!(!test_game.validate_password("iamlove"));
    assert!(!test_game.validate_password(""));
}
