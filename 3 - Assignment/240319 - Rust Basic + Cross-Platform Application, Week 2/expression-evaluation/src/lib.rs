// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    // An operation on two subexpressions.
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    // A literal value
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Value(ret) => Ok(ret),
        Expression::Op { op, left, right } => {
            let left = eval(*left)?;
            let right = eval(*right)?;
            if let (Operation::Div, 0) = (&op, right) {
                return Err(String::from("Division by zero"));
            }
            let ret = match op {
                Operation::Add => left.checked_add(right),
                Operation::Sub => left.checked_sub(right),
                Operation::Mul => left.checked_mul(right),
                Operation::Div => left.checked_div(right),
            };
            ret.ok_or(String::from("Integer overflow"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(eval(Expression::Value(19)), Ok(19));
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(Expression::Value(10)),
                right: Box::new(Expression::Value(20)),
            }),
            Ok(30)
        );
    }

    #[test]
    fn test_recursion() {
        let term1 = Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(9)),
        };
        let term2 = Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Op {
                op: Operation::Sub,
                left: Box::new(Expression::Value(3)),
                right: Box::new(Expression::Value(4)),
            }),
            right: Box::new(Expression::Value(5)),
        };
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(term1),
                right: Box::new(term2),
            }),
            Ok(85)
        );
    }

    #[test]
    fn test_error() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Div,
                left: Box::new(Expression::Value(99)),
                right: Box::new(Expression::Value(0)),
            }),
            Err(String::from("Division by zero"))
        );
    }

    #[test]
    fn test_overflow() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Mul,
                left: Box::new(Expression::Value(i64::MAX)),
                right: Box::new(Expression::Value(1)),
            }),
            Ok(i64::MAX),
        );
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Mul,
                left: Box::new(Expression::Value(i64::MAX)),
                right: Box::new(Expression::Value(2)),
            }),
            Err(String::from("Integer overflow"))
        );
    }
}
