use frontend::Position;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum TokenType {
    // Types:
    INTEGER,
    REAL,
    BOOL,
    STRING,
    CHAR,
    // Literals:
    KEYWORD,
    SYMBOL,
    // Delimiters:
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACK,
    RBRACK,
    COMMA,
    // Operators:
    OPPLUS,
    OPMINUS,
    OPSTAR,
    OPSLASH,
    OPEQUAL,
    OPBANG,
    // Other:
    EOL,
    EOF,
}
