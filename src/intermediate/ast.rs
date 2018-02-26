// The data we will visit.
pub enum Statement {
    Assignment(Identifier, Expression),
    Constant((Identifier, Expression)),
    Variable((Identifier, Expression)),
    Expression(Expression),
}

pub struct Identifier {
    name: String,
}

pub enum Expression {
    Nil,
    Integer(i64),
    Real(f64),
    String(String),
    Boolean(bool),
    BinaryOperation(BinaryOperator, Box<Expression>, Box<Expression>),
    UnaryOperation(UnaryOperator, Box<Expression>),
}

pub enum BinaryOperator {
    Assign,
    /* Math operators: */
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    /* Compare operators: */
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    /* Logical operators: */
    And,
    Or,
}

pub enum UnaryOperator {
    Not,
}
