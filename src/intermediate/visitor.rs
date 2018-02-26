// The abstract visitor.
use intermediate::ast::*;

pub trait Visitor<T> {
    fn visit_identifier(&mut self, n: &Identifier) -> T;
    fn visit_statement(&mut self, s: &Statement) -> T;
    fn visit_expression(&mut self, e: &Expression) -> T;
}