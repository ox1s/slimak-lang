use token::TokenType;

mod lexer {
    #[test]
    fn TestNextToken() {
        let input = "=+(){};";

        struct Data {
            expectedType: token::TokenType,
            expectedLiteral: String,
        }
        let tests = vec![
            Data { expectedType: token.ASSIGN, "="},
            Data { expectedType: token.PLUS, "+"},
            Data { expectedType: token.LPAREN, "("},
            Data { expectedType: token.RPAREN, ")"},
            Data { expectedType: token.LBRACE, "{"},
            Data { expectedType: token.RBRACE, "}"},
            Data { expectedType: token.COMMA, ","},
            Data { expectedType: token.SEMICOLON, ";"},
            Data { expectedType: token.EOF, ""}
        ];
        let l = New(input);
        io::stdin().read_line(&mut input)?;

        for (i, tt) in tests.iter().enumerate() {
            let tok = l.NextToken();

            assert_eq!(tok.kind != tt.expectedType);
            assert_eq!(tok.literal != tt.expecterLiteral);
        }
    }
}
