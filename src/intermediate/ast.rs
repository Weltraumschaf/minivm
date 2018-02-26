// The data we will visit.
pub enum Stmt {
    Expr(Expr),
    Let(Name, Expr),
}

pub struct Name {
    value: String,
}

pub enum Expr {
    Integer(i64),
    Real(f64),
    String(String),
    Boolean(bool),
    Addition(Box<Expr>, Box<Expr>),
    Subtraction(Box<Expr>, Box<Expr>),
    Multiplication(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
    Modulo(Box<Expr>, Box<Expr>),
}