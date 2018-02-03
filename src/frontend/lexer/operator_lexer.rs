use frontend::lexer::SubLexer;
use frontend::token::*;
use frontend::character_stream::CharacterStream;
use frontend::character_helper::CharacterHelper;
#[cfg(test)]
use frontend::Position;

pub struct OperatorLexer {}

impl OperatorLexer {
    pub fn new() -> OperatorLexer {
        OperatorLexer {}
    }
}

impl SubLexer for OperatorLexer {
    fn scan(&self, input: &mut CharacterStream) -> Token {
        panic!("not implemented yet: operator");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;
}