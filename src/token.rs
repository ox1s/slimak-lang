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
}

pub fn value_of_token_type(token_type: TokenType) -> &'static str {
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
        TokenType::Slash => "-",
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
    }
}

pub struct Token {
    pub type_of: TokenType,
    pub literal: String,
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "няхай" => TokenType::Let,
        _ => TokenType::Ident,
    }
}

// pub const ILLEGAL: &str = "ILLEGAL";
// pub const EOF: &str = "EOF";

// pub const IDENT: &str = "IDENT";
// pub const INT: &str = "INT";

// pub const ASSIGN: &str = "=";
// pub const PLUS: &str = "+";
// pub const MINUS: &str = "-";
// pub const BANG: &str = "!";
// pub const ASTERISK: &str = "*";
// pub const SLASH: &str = "/";

// pub const LT: &str = "<";
// pub const GT: &str = ">";

// pub const EQ: &str = "==";
// pub const NOT_EQ: &str = "!=";

// pub const SEMICOLON: &str = ";";
// pub const COMMA: &str = ",";

// pub const LPAREN: &str = "(";
// pub const RPAREN: &str = ")";
// pub const LBRACE: &str = "{";
// pub const RBRACE: &str = "}";

// pub const FUNCTION: &str = "FUNCTION";
// pub const LET: &str = "LET";
