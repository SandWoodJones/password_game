use super::*;

#[test]
fn rule_1_test() {
    let test_game = PasswordGame::new();
    assert!(test_game.validate_password("abcde", GameRules::FiveChars));
    assert!(test_game.validate_password("abcdefghij", GameRules::FiveChars));
    assert!(!test_game.validate_password("abcd", GameRules::FiveChars));
}

#[test]
fn rule_2_test() {
    let test_game = PasswordGame::new();
    assert!(test_game.validate_password("5", GameRules::IncludeNum));
    assert!(test_game.validate_password("foo9", GameRules::IncludeNum));
    assert!(test_game.validate_password("30\u{0BF0}foo", GameRules::IncludeNum));
    assert!(!test_game.validate_password("foo", GameRules::IncludeNum));
    assert!(!test_game.validate_password("\u{0BF0}\u{2169}\u{516D}", GameRules::IncludeNum));
}

#[test]
fn rule_3_test() {
    let test_game = PasswordGame::new();
    assert!(test_game.validate_password("A", GameRules::IncludeUpper));
    assert!(test_game.validate_password("abcdeFgHiJ", GameRules::IncludeUpper));
    assert!(!test_game.validate_password("a", GameRules::IncludeUpper));
    assert!(!test_game.validate_password("abcdefghij", GameRules::IncludeUpper));
}

#[test]
fn rule_4_test() {
    let test_game = PasswordGame::new();
    assert!(test_game.validate_password("@", GameRules::IncludeSpecial));
    assert!(test_game.validate_password("a.bcd-ef@", GameRules::IncludeSpecial));
    assert!(test_game.validate_password("ççç", GameRules::IncludeSpecial));
    assert!(test_game.validate_password("\u{0436}", GameRules::IncludeSpecial));
    assert!(!test_game.validate_password("abc", GameRules::IncludeSpecial));
    assert!(!test_game.validate_password("3000AAAAF", GameRules::IncludeSpecial));
}

#[test]
fn rule_5_test() {
    let test_game = PasswordGame::new();
    assert!(test_game.validate_password("997", GameRules::AddTo25));
    assert!(test_game.validate_password("999999999999", GameRules::AddTo25));
    assert!(test_game.validate_password("a5bcd5.-5@@5AA5", GameRules::AddTo25));
    assert!(!test_game.validate_password("996", GameRules::AddTo25));
    assert!(!test_game.validate_password("5bd5.@5AA5", GameRules::AddTo25));
}

#[test]
fn rule_6_test() {
    let test_game = PasswordGame::new();
    assert!(test_game.validate_password("january", GameRules::IncludeMonth));
    assert!(test_game.validate_password("00--dEcEmbER,.", GameRules::IncludeMonth));
    assert!(!test_game.validate_password("januaaaaary", GameRules::IncludeMonth));
    assert!(!test_game.validate_password("apil", GameRules::IncludeMonth));
}

#[test]
fn rule_7_test() {
    let test_game = PasswordGame::new();
    assert!(test_game.validate_password("V", GameRules::IncludeRoman));
    assert!(test_game.validate_password("MILLION", GameRules::IncludeRoman));
    assert!(test_game.validate_password("abcdefghIj", GameRules::IncludeRoman));
    assert!(!test_game.validate_password("million", GameRules::IncludeRoman));
}

#[test]
fn rule_8_test() {
    let test_game = PasswordGame::new();
    todo!()
}

#[test]
fn rule_9_test() {
    let test_game = PasswordGame::new();
    todo!()
}

#[test]
fn rule_10_test() {
    let test_game = PasswordGame::new();
    todo!()
}

#[test]
fn rule_11_test() {
    let test_game = PasswordGame::new();
    todo!()
}

#[test]
fn rule_12_test() {
    let test_game = PasswordGame::new();
    todo!()
}

#[test]
fn rule_13_test() {
    let test_game = PasswordGame::new();
    todo!()
}

#[test]
fn rule_14_test() {
    let test_game = PasswordGame::new();
    todo!()
}

#[test]
fn rule_15_test() {
    let test_game = PasswordGame::new();
    assert!(test_game.validate_password("abcd0ef", GameRules::IncludeLeapYear));
    assert!(test_game.validate_password("20AAAAA", GameRules::IncludeLeapYear));
    assert!(test_game.validate_password("2123124", GameRules::IncludeLeapYear));
    assert!(!test_game.validate_password("a-~30e..ç", GameRules::IncludeLeapYear));
    assert!(!test_game.validate_password("2200", GameRules::IncludeLeapYear));
}

#[test]
fn rule_16_test() {
    let test_game = PasswordGame::new();
    todo!()
}

#[test]
fn rule_17_test() {
    let test_game = PasswordGame::new();
    todo!()
}
