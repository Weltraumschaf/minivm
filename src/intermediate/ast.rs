// The data we will visit.
pub enum Statement {
    Assignment(Identifier, Expression),
    Expression(Expression),
}

pub struct Identifier {
    name: String,
}

pub enum Expression {
    Integer(i64),
    Real(f64),
    String(String),
    Boolean(bool),
    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Modulo(Box<Expression>, Box<Expression>),
}