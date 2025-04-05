#![allow(clippy::all, dead_code)]

type Identifier = String;
type Value = i64;

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
enum Statement {
    Operation {
        lhs: Box<Statement>,
        rhs: Box<Statement>,
        op: Operator,
    },
    Litteral(Value),
}

impl Statement {
    fn eval(&self) -> i64 {
        match self {
            Statement::Operation { lhs, rhs, op } => {
                let l = lhs.eval();
                let r = rhs.eval();
                match op {
                    Operator::Add => l + r,
                    Operator::Sub => l - r,
                    Operator::Mul => l * r,
                    Operator::Div => l / r,
                    Operator::Mod => l % r,
                }
            }
            Statement::Litteral(l) => *l,
        }
    }
}

fn main() {}

fn v(val: i64) -> Statement {
    Statement::Litteral(val)
}

fn op(o: Operator, l: Statement, r: Statement) -> Statement {
    Statement::Operation {
        lhs: Box::new(l),
        rhs: Box::new(r),
        op: o,
    }
}

#[cfg(test)]
mod tests {
    use crate::Operator::*;
    use crate::*;

    #[test]
    fn add() {
        // 1 + 1
        assert_eq!(op(Add, v(1), v(1)).eval(), 2);
    }

    #[test]
    fn sub() {
        // 2 - 4
        assert_eq!(op(Sub, v(2), v(4)).eval(), -2);
    }

    #[test]
    fn mul() {
        // 4 * 3
        assert_eq!(op(Mul, v(4), v(3)).eval(), 12);
    }

    #[test]
    fn div() {
        // 7 / 2
        assert_eq!(op(Div, v(7), v(2)).eval(), 3);
    }

    #[test]
    fn modulo() {
        // 8 % 3
        assert_eq!(op(Mod, v(8), v(3)).eval(), 2);
    }

    #[test]
    fn complex_1() {
        // (2 + 3) * 5
        assert_eq!(op(Mul, op(Add, v(2), v(3)), v(5)).eval(), 25);
    }

    #[test]
    fn complex_2() {
        // ((3 + 4) - ((5 * 3) % 2)
        assert_eq!(
            op(Sub, op(Add, v(3), v(4)), op(Mod, op(Mul, v(5), v(3)), v(2))).eval(),
            6
        );
    }
}
