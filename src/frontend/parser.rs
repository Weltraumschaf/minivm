use frontend::lexer::Lexer;
use frontend::token::TokenType;
use frontend::token::Keyword;
use frontend::token::Operator;

/// Parses the tokens recognized by the lexer.
pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer }
    }

    pub fn parse(&self) {
        loop {
            if *self.lexer.current().get_token_type() == TokenType::EOL {
                break;
            }

            self.parse_statement();
        }
    }

    fn parse_statement(&self) {
        match *self.lexer.current().get_token_type() {
            TokenType::Keyword(ref keyword) => {
                match *keyword {
                    Keyword::Const => self.parse_constant_declaration(),
                    Keyword::Var => self.parse_variable_declaration(),
                    _ => panic!("Unexpected keyword: {}!", keyword),
                }
            },
            TokenType::Identifier(_) => {
                match *self.lexer.peek().get_token_type() {
                    TokenType::Operator(Operator::Assign) => self.parse_assignment(),
                    _ => self.parse_or_expression(),
                }
            },
            _ => panic!("Unexpected token: {}!", self.lexer.current()),
        }
    }

    fn parse_assignment(&self) {
        unimplemented!();
    }

    fn parse_constant_declaration(&self) {
        unimplemented!();
    }

    fn parse_variable_declaration(&self) {
        unimplemented!();
    }

    fn parse_or_expression(&self) {
        unimplemented!();
    }
}