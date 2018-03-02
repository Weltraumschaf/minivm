use std::fmt;
use frontend::Position;

/// Sentinel to indicate end of file.
pub const END_OF_FILE: char = 0 as char;

/// Defines a recognized token in the source.
#[derive(Debug, PartialEq)]
pub struct Token {
    position: Position,
    token_type: TokenType,
    literal: String,
}

impl Token {
    /// Creates a new token.
    pub fn new(position: Position, token_type: TokenType, literal: String) -> Token {
        Token { position, token_type, literal }
    }

    /// Get the type of the token.
    pub fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} '{}' {}>", self.token_type, self.literal, self.position)
    }
}

/// Types of tokens.
#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Types:
    INTEGER(i64),
    REAL(f64),
    BOOL(bool),
    STRING(String),
    CHARACTER(char),
    // Literals:
    KEYWORD(Keyword),
    IDENTIFIER(String),
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


impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Reserved keywords.
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

impl Keyword {
    /// Maps a literal keyword.
    pub fn for_literal(literal: &String) -> Keyword {
        match literal.as_ref() {
            "and" => Keyword::AND,
            "or" => Keyword::OR,
            "not" => Keyword::NOT,
            "if" => Keyword::IF,
            "else" => Keyword::ELSE,
            "while" => Keyword::WHILE,
            "const" => Keyword::CONST,
            "var" => Keyword::VAR,
            _ => panic!("Unrecognized keyword '{}'!", literal),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Available operators.
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

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}