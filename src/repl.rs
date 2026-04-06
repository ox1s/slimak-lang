use crate::{
    lexer::Lexer,
    token::{Token, TokenType},
};
use std::io::{Write, stdin, stdout};
const PROMT: &str = ">> ";

pub fn start() {
    loop {
        let mut line = String::new();
        print!("{}", &PROMT);

        stdout().flush().expect("Failed to flush stdout");

        let _ = stdin().read_line(&mut line);

        if line == "\0" {
            return;
        }

        let mut l = Lexer::new(line);
        let mut tok: Token;

        tok = l.next_token();
        while tok.type_of != TokenType::Eof {
            println!("{}", tok);
            tok = l.next_token();
        }
    }
}
