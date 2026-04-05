use crate::lexer::Lexer;
use crate::token::TokenType;
use rstest::rstest;

#[rstest]
#[case(TokenType::Assign, "=", "=")]
#[case(TokenType::Plus, "+", "+")]
#[case(TokenType::LParen, "(", "(")]
#[case(TokenType::RParen, ")", ")")]
#[case(TokenType::LBrace, "{", "{")]
#[case(TokenType::RBrace, "}", "}")]
#[case(TokenType::Comma, ",", ",")]
#[case(TokenType::Semicolon, ";", ";")]
#[case(TokenType::Eof, "\0", "\0")]
fn next_token_test(
    #[case] expected_type: TokenType,
    #[case] expected_literal: String,
    #[case] input_str: String,
) {
    // Мы должны взять по одному символу в строке input_str
    // И создать с этого lexer
    // lexer создаст токен у которого будет такой тип и строка

    let mut l = Lexer::new(input_str);

    let t = l.next_token();

    assert_eq!(expected_type, t.type_of);
    assert_eq!(expected_literal, t.literal);
}
#[test]
fn test_complex_input() {
    let input = "няхай пяць = 5;
няхай дзесяць = 10;
няхай дадаць = праца(а, б)
{
   а + б;
};

няхай вынiк = дадаць(пяць, дзесяць);
!-/*5
5 < 10 > 5;
";

    let tests = vec![
        (TokenType::Let, "няхай"),
        (TokenType::Ident, "пяць"),
        (TokenType::Assign, "="),
        (TokenType::Int, "5"),
        (TokenType::Semicolon, ";"),
        (TokenType::Let, "няхай"),
        (TokenType::Ident, "дзесяць"),
        (TokenType::Assign, "="),
        (TokenType::Int, "10"),
        (TokenType::Semicolon, ";"),
        (TokenType::Let, "няхай"),
        (TokenType::Plus, "дадаць"),
        (TokenType::Assign, "="),
        (TokenType::Function, "праца"),
        (TokenType::LParen, "("),
        (TokenType::Ident, "а"),
        (TokenType::Comma, ","),
        (TokenType::Ident, "б"),
        (TokenType::RParen, ")"),
        (TokenType::LBrace, "{"),
        (TokenType::Ident, "а"),
        (TokenType::Plus, "+"),
        (TokenType::Ident, "б"),
        (TokenType::Semicolon, ";"),
        (TokenType::LBrace, "}"),
        (TokenType::Semicolon, ";"),
    ];

    let mut l = Lexer::new(input.to_string());

    for (i, (expected_type, expected_literal)) in tests.into_iter().enumerate() {
        let tok = l.next_token();

        assert_eq!(tok.type_of, expected_type, "Тест [{}] провален по типу", i);
        assert_eq!(
            tok.literal, expected_literal,
            "Тест [{}] провален по литералу",
            i
        );
    }
}
