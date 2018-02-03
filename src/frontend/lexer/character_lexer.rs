use frontend::lexer::SubLexer;
use frontend::token::*;
use frontend::character_stream::CharacterStream;
use frontend::character_helper::CharacterHelper;
#[cfg(test)]
use frontend::Position;

pub struct CharacterLexer {}

impl CharacterLexer {
    pub fn new() -> CharacterLexer {
        CharacterLexer {}
    }
}

impl SubLexer for CharacterLexer {
    fn scan(&self, input: &mut CharacterStream) -> Token {
        panic!("not implemented yet: character literal");
    }
}