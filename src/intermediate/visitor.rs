// The abstract visitor.
use intermediate::ast::*;

pub trait Visitor<T> {
    fn visit_identifier(&mut self, n: &Identifier) -> T;
    fn visit_statement(&mut self, s: &Statement) -> T;
    fn visit_expression(&mut self, e: &Expression) -> T;
}

// An example concrete implementation - walks the AST interpreting it as code.
pub struct Interpreter;

impl Visitor<i64> for Interpreter {
    fn visit_identifier(&mut self, n: &Identifier) -> i64 { panic!() }

    fn visit_statement(&mut self, s: &Statement) -> i64 {
        match *s {
            Statement::Expression(ref e) => self.visit_expression(e),
            Statement::Assignment(..) => unimplemented!(),
            _ => unimplemented!(),
        }
    }

    fn visit_expression(&mut self, e: &Expression) -> i64 {
        match *e {
            Expression::Integer(n) => n,
            Expression::BinaryOperation(ref op, ref lhs, ref rhs) => {
                self.visit_expression(lhs) + self.visit_expression(rhs)
            },
            _ => unimplemented!(),
        }
    }
}