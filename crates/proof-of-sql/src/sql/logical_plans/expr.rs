use super::LogicalPlanError;
use crate::base::database::{ColumnRef, LiteralValue};
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};
use sqlparser::ast;

/// Enum of column expressions that are either provable or supported in postprocessing
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    /// Column
    Column(ColumnRef),
    /// A constant expression
    Literal(LiteralValue),
    /// Binary operation
    Binary {
        /// Left hand side of the binary operation
        left: Box<Expr>,
        /// Right hand side of the binary operation
        right: Box<Expr>,
        /// Binary operator
        op: BinaryOperator,
    },
    /// NOT expression
    Not(Box<Expr>),
}

/// Enum of binary operators we support
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    /// Equals
    Eq,
    /// Not equals
    NotEq,
    /// Greater than
    Gt,
    /// Less than
    Lt,
    /// Greater than or equals
    GtEq,
    /// Less than or equals
    LtEq,
    /// Logical AND
    And,
    /// Logical OR
    Or,
    /// Plus
    Plus,
    /// Minus
    Minus,
    /// Multiply
    Multiply,
    /// Divide
    Divide,
}

impl TryFrom<ast::BinaryOperator> for BinaryOperator {
    type Error = LogicalPlanError;

    fn try_from(op: ast::BinaryOperator) -> Result<Self, Self::Error> {
        match op {
            ast::BinaryOperator::Eq => Ok(BinaryOperator::Eq),
            ast::BinaryOperator::NotEq => Ok(BinaryOperator::NotEq),
            ast::BinaryOperator::Gt => Ok(BinaryOperator::Gt),
            ast::BinaryOperator::Lt => Ok(BinaryOperator::Lt),
            ast::BinaryOperator::GtEq => Ok(BinaryOperator::GtEq),
            ast::BinaryOperator::LtEq => Ok(BinaryOperator::LtEq),
            ast::BinaryOperator::And => Ok(BinaryOperator::And),
            ast::BinaryOperator::Or => Ok(BinaryOperator::Or),
            ast::BinaryOperator::Plus => Ok(BinaryOperator::Plus),
            ast::BinaryOperator::Minus => Ok(BinaryOperator::Minus),
            ast::BinaryOperator::Multiply => Ok(BinaryOperator::Multiply),
            ast::BinaryOperator::Divide => Ok(BinaryOperator::Divide),
            _ => Err(LogicalPlanError::UnsupportedBinaryOperator { op }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Binary operators
    #[test]
    fn we_can_convert_supported_sqlparser_binary_operators() {
        // Let's test all our supported binary operators.
        let test_cases = vec![
            (ast::BinaryOperator::Eq, BinaryOperator::Eq),
            (ast::BinaryOperator::NotEq, BinaryOperator::NotEq),
            (ast::BinaryOperator::Gt, BinaryOperator::Gt),
            (ast::BinaryOperator::Lt, BinaryOperator::Lt),
            (ast::BinaryOperator::GtEq, BinaryOperator::GtEq),
            (ast::BinaryOperator::LtEq, BinaryOperator::LtEq),
            (ast::BinaryOperator::And, BinaryOperator::And),
            (ast::BinaryOperator::Or, BinaryOperator::Or),
            (ast::BinaryOperator::Plus, BinaryOperator::Plus),
            (ast::BinaryOperator::Minus, BinaryOperator::Minus),
            (ast::BinaryOperator::Multiply, BinaryOperator::Multiply),
            (ast::BinaryOperator::Divide, BinaryOperator::Divide),
        ];

        for (sql_op, expected) in test_cases {
            let result = BinaryOperator::try_from(sql_op).unwrap();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn we_cannot_convert_unsupported_sqlparser_binary_operators() {
        // Let's test an unsupported operator.
        let unsupported_op = ast::BinaryOperator::Spaceship;
        let result = BinaryOperator::try_from(unsupported_op);
        assert!(matches!(
            result,
            Err(LogicalPlanError::UnsupportedBinaryOperator { .. })
        ));
    }
}
