use frontend::token::Token;
use frontend::token::TokenType::*;
use frontend::Position;
use frontend::lexer::SubLexer;
use frontend::lexer::Lexer;

pub struct NumberLexer {}

impl NumberLexer {
    pub fn new() -> NumberLexer {
        NumberLexer{}
    }
}

impl SubLexer for NumberLexer  {
    fn scan(&self, parent: &Lexer) -> Token {
        Token::new(Position::null(), EOL, String::from(""))
    }
}