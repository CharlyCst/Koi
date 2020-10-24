use super::*;

#[test]
fn ignores_whitespace() {
    let scanner = Scanner::new("    \r\n.");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![Token::Dot]);
}

#[test]
fn scans_empty_string() {
    let scanner = Scanner::new("");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![]);
}

#[test]
fn scans_keyword() {
    let scanner = Scanner::new("while");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![Token::While]);
}

#[test]
fn stops_scanning() {
    let scanner = Scanner::new(".");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![Token::Dot]);
}

#[test]
#[should_panic]
fn panics_unexpected_symbol() {
    let mut scanner = Scanner::new("§");
    scanner.next();
}

#[test]
fn scans_identifier() {
    let scanner = Scanner::new("columbia");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![Token::Identifier]);
}

#[test]
fn scans_complex_string() {
    use Token::*;

    let scanner = Scanner::new("for.while:return cc whine&&!==++--break,continue\
    ;(){}[]$()exp+=-=*=/=%=^=^%*/-+true:h;false!nil?var/if else;fn::== =for");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![
        For, Dot, While, Colon, Return, Identifier, Identifier, AmperAmper, BangEqual, Equal, PlusPlus,
        MinusMinus, Break, Comma, Continue, Semicolon, LeftParen, RightParen, LeftBrace, RightBrace,
        LeftBracket, RightBracket, DollarLeftParen, RightParen, Exp, PlusEqual, MinusEqual, StarEqual,
        SlashEqual, PercEqual, CaretEqual, Caret, Perc, Star, Slash, Minus, Plus, True, Colon, Identifier,
        Semicolon, False, Bang, Nil, Question, Var, Slash, If, Else, Semicolon, Fn, Colon, Colon,
        EqualEqual, Equal, For
    ]);
}
