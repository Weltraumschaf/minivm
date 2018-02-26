use frontend::lexer::Lexer;

/// Parses the tokens recognized by the lexer.
pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer }
    }

    pub fn parse(&self) {

    }
}