use frontend::Position;

pub const END_OF_FILE: char = 0 as char;

#[derive(Debug, PartialEq)]
pub struct Token {
    pos: Position,
    token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(pos: Position, token_type: TokenType, literal: String) -> Token {
        Token { pos, token_type, literal }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Types:
    INTEGER(i64),
    REAL(f64),
    BOOL(bool),
    STRING(String),
    CHAR(char),
    // Literals:
    KEYWORD(Keyword),
    SYMBOL(String),
    // Delimiters:
    L_PAREN,
    R_PAREN,
    L_BRACE,
    R_BRACE,
    L_BRACK,
    R_BRACK,
    COMMA,
    // Operators:
    OPERATOR(Operator),
    // Other:
    EOL,
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    AND,
    OR,
    NOT,
    IF,
    ELSE,
    WHILE,
    CONST,
    VAR,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    ASSIGN,
    // compare operators
    EQUAL,
    NOT_EQUAL,
    LESS_THAN,
    LESS_THAN_EQUAL,
    GREATER_THAN,
    GREATER_THAN_EQUAL,
    // math operators
    PLUS,
    MINUS,
    STAR,
    SLASH,
    MOD,
}