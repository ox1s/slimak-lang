use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,
    Semicolon,
    Comma,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    If,
    Else,
    Return,
    True,
    False,
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
fn value_of_token_type(token_type: TokenType) -> &'static str {
    match token_type {
        TokenType::Illegal => "НЕЗАКОННА", // незаконна
        TokenType::Eof => "КР",            // канец радка
        TokenType::Ident => "ІДЭНТ",       // ідэнт
        TokenType::Assign => "=",
        TokenType::Int => "ЦЭЛЫ", // цэлы
        TokenType::Plus => "+",
        TokenType::Minus => "-",
        TokenType::Bang => "!",
        TokenType::Asterisk => "*",
        TokenType::Slash => "/",
        TokenType::Lt => "<",     // менш чым
        TokenType::Gt => ">",     // больш чым
        TokenType::Eq => "==",    //роўны
        TokenType::NotEq => "!=", //ня роўны
        TokenType::Semicolon => ";",
        TokenType::Comma => ",",
        TokenType::LParen => "(",
        TokenType::RParen => ")",
        TokenType::LBrace => "{",
        TokenType::RBrace => "}",
        TokenType::Function => "ФУНКЦЫЯ",
        TokenType::Let => "НЯХАЙ",
        TokenType::If => "КАЛI",
        TokenType::Else => "IНШ",
        TokenType::Return => "ВЯРТАЦЬ",
        TokenType::True => "ПРАЎДА",
        TokenType::False => "ХЛУСНЯ",
    }
}

pub struct Token {
    pub type_of: TokenType,
    pub literal: String,
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.type_of, self.literal)
    }
}
pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "няхай" => TokenType::Let,
        "праца" => TokenType::Function,
        "праўда" => TokenType::True,
        "хлусня" => TokenType::False,
        "калi" => TokenType::If,
        "інш" => TokenType::Else,
        "вяртаць" => TokenType::Return,
        _ => TokenType::Ident,
    }
}
