// https://github.com/rust-unofficial/patterns/blob/master/patterns/visitor.md

mod ast;
mod visitor;

use intermediate::visitor::*;
use intermediate::ast::*;

// An example concrete implementation - walks the AST interpreting it as code.
struct Interpreter;

impl Visitor<i64> for Interpreter {
    fn visit_identifier(&mut self, n: &Identifier) -> i64 { panic!() }

    fn visit_statement(&mut self, s: &Statement) -> i64 {
        match *s {
            Statement::Expression(ref e) => self.visit_expression(e),
            Statement::Assignment(..) => unimplemented!(),
        }
    }

    fn visit_expression(&mut self, e: &Expression) -> i64 {
        match *e {
            Expression::Integer(n) => n,
            Expression::Addition(ref lhs, ref rhs) => self.visit_expression(lhs) + self.visit_expression(rhs),
            Expression::Subtraction(ref lhs, ref rhs) => self.visit_expression(lhs) - self.visit_expression(rhs),
            _ => unimplemented!(),
        }
    }
}