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
    Integer(i64),
    Real(f64),
    Bool(bool),
    String(String),
    Character(char),
    // Literals:
    Keyword(Keyword),
    Identifier(String),
    // Delimiters:
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    // Operators:
    Operator(Operator),
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
    And,
    Or,
    Not,
    If,
    Else,
    While,
    Const,
    Var,
}

impl Keyword {
    /// Maps a literal keyword.
    pub fn for_literal(literal: &String) -> Keyword {
        match literal.as_ref() {
            "and" => Keyword::And,
            "or" => Keyword::Or,
            "not" => Keyword::Not,
            "if" => Keyword::If,
            "else" => Keyword::Else,
            "while" => Keyword::While,
            "const" => Keyword::Const,
            "var" => Keyword::Var,
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
    Assign,
    // compare operators
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    // math operators
    Plus,
    Minus,
    Star,
    Slash,
    Mod,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}