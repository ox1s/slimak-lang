use crate::token::{self, Token, TokenType};

pub struct Lexer {
    pub input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

impl Lexer {
    // &mut self - мы изменяем позицию
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        return l;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0'
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1
    }

    pub fn next_token(&mut self) -> token::Token {
        let mut tok = token::Token {
            type_of: TokenType::Eof,
            literal: '\0'.to_string(),
        };

        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    // TODO: Ошибка
                    let ch1 = self.ch;
                    self.read_char();
                    let literal = ch1.to_string() + &String::from(self.ch);
                    tok = token::Token {
                        type_of: token::TokenType::Eq,
                        literal: literal,
                    }
                }
                tok = self.new_token(token::TokenType::Assign, self.ch)
            }
            ';' => tok = self.new_token(token::TokenType::Semicolon, self.ch),
            '(' => tok = self.new_token(token::TokenType::LParen, self.ch),
            ')' => tok = self.new_token(token::TokenType::RParen, self.ch),
            ',' => tok = self.new_token(token::TokenType::Comma, self.ch),
            '+' => tok = self.new_token(token::TokenType::Plus, self.ch),
            '{' => tok = self.new_token(token::TokenType::LBrace, self.ch),
            '}' => tok = self.new_token(token::TokenType::RBrace, self.ch),
            '/' => tok = self.new_token(token::TokenType::Slash, self.ch),
            '-' => tok = self.new_token(token::TokenType::Minus, self.ch),
            '>' => tok = self.new_token(token::TokenType::Gt, self.ch),
            '<' => tok = self.new_token(token::TokenType::Lt, self.ch),
            '!' => tok = self.new_token(token::TokenType::Bang, self.ch),
            '*' => tok = self.new_token(token::TokenType::Asterisk, self.ch),
            '\0' => {
                tok.type_of = token::TokenType::Eof;
                tok.literal = '\0'.to_string();
            }
            _ => {
                if self.ch.is_alphabetic() {
                    tok.literal = self.read_identifier().iter().collect();
                    tok.type_of = token::lookup_ident(&tok.literal);
                    return tok;
                } else if self.ch.is_numeric() {
                    tok.type_of = TokenType::Int;
                    tok.literal = self.read_number().iter().collect();
                    return tok;
                } else {
                    tok = self.new_token(TokenType::Illegal, self.ch)
                }
            }
        }

        self.read_char();
        tok
    }

    fn peek_char(self) -> char {
        if self.read_position >= self.input.len() {
            '0'
        } else {
            self.input[self.read_position]
        }
    }

    fn read_number(&mut self) -> Vec<char> {
        // only integers
        let positon = self.position;

        while self.ch.is_numeric() {
            self.read_char();
        }

        let slice = &self.input[positon..=self.position - 1];
        slice.to_vec()
    }
    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
    pub fn read_identifier(&mut self) -> Vec<char> {
        let positon = self.position;

        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char();
        }

        let slice = &self.input[positon..=self.position - 1];
        slice.to_vec()
        //l.input[position:l.position] - берем элементы с начала до position (конца букв)
    }
    pub fn new_token(&self, token_type: token::TokenType, ch: char) -> Token {
        Token {
            type_of: token_type,
            literal: ch.to_string(),
        }
    }
}
