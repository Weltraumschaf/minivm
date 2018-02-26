// The abstract visitor.
use intermediate::ast::*;

pub trait Visitor<T> {
    fn visit_name(&mut self, n: &Name) -> T;
    fn visit_stmt(&mut self, s: &Stmt) -> T;
    fn visit_expr(&mut self, e: &Expr) -> T;
}