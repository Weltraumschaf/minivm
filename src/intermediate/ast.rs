/// The base building block are statements.
pub enum Statement {
    /// Assignment statement: `identifier = expression`.
    Assignment(Identifier, Expression),
    /// Constant declaration: `const identifier = expression`.
    Constant((Identifier, Expression)),
    /// Variable declaration: `var identifier` or `var identifier = expression`.
    Variable((Identifier, Expression)),
    /// Expression statement.
    Expression(Expression),
}

/// Defines an identifier.
pub struct Identifier {
    /// Name of the identifier.
    name: String,
}

/// The various expressions.
pub enum Expression {
    /// Nil expression used e.g. for variable declaration without initial value.
    Nil,
    /// Integer literal expression.
    Integer(i64),
    /// Real literal expression.
    Real(f64),
    /// String literal expression.
    String(String),
    /// Boolean literal expression.
    Boolean(bool),
    /// Binary operation expression.
    BinaryOperation(BinaryOperator, Box<Expression>, Box<Expression>),
    /// Unary operation expression.
    UnaryOperation(UnaryOperator, Box<Expression>),
}

/// Binary operators.
pub enum BinaryOperator {
    /// Assign operator.
    Assign,
    // Math operators:
    /// Add operator.
    Add,
    /// Subtract operator.
    Subtract,
    /// Multiply operator.
    Multiply,
    /// Divide operator.
    Divide,
    /// Modulo operator.
    Modulo,
    // Compare operators:
    /// Equal operator.
    Equal,
    /// Not equal operator.
    NotEqual,
    /// Less than operator.
    LessThan,
    /// Less than or equal operator.
    LessThanEqual,
    /// Greater than operator.
    GreaterThan,
    /// Greater than or equal operator.
    GreaterThanEqual,
    // Logical operators:
    /// Logical and operator.
    And,
    /// Logical or operator.
    Or,
}

/// Unary operators.
pub enum UnaryOperator {
    /// Logical not operator.
    Not,
}
