// https://github.com/rust-unofficial/patterns/blob/master/patterns/visitor.md

mod ast;
mod visitor;

use intermediate::visitor::*;
use intermediate::ast::*;

// An example concrete implementation - walks the AST interpreting it as code.
struct Interpreter;
impl Visitor<i64> for Interpreter {
    fn visit_name(&mut self, n: &Name) -> i64 { panic!() }
    fn visit_stmt(&mut self, s: &Stmt) -> i64 {
        match *s {
            Stmt::Expr(ref e) => self.visit_expr(e),
            Stmt::Let(..) => unimplemented!(),
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> i64 {
        match *e {
            Expr::Integer(n) => n,
            Expr::Addition(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
            Expr::Subtraction(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
            _ => panic!("Not implemented!"),
        }
    }
}