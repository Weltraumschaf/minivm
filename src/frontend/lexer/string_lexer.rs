use frontend::lexer::SubLexer;
use frontend::token::*;
use frontend::character_stream::CharacterStream;
use frontend::character_helper::CharacterHelper;
#[cfg(test)]
use frontend::Position;

pub struct StringLexer {}

impl StringLexer {
    pub fn new() -> StringLexer {
        StringLexer {}
    }
}

impl SubLexer for StringLexer {
    fn scan(&self, input: &mut CharacterStream) -> Token {
        panic!("not implemented yet: string literal");
    }
}