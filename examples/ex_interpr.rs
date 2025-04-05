#![allow(clippy::all, dead_code)]

use std::collections::HashMap;

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
    Assign {
        name: Identifier,
        value: Box<Statement>,
    },
    Variable(Identifier),
    Operation {
        lhs: Box<Statement>,
        rhs: Box<Statement>,
        op: Operator,
    },
    Litteral(Value),
    Sequence(Box<Statement>, Box<Statement>),
    If {
        cond: Box<Statement>,
        case_true: Box<Statement>,
        case_false: Box<Statement>,
    },
    While {
        cond: Box<Statement>,
        body: Box<Statement>,
    },
}

impl Statement {
    fn eval_empty(&self) -> i64 {
        self.eval(&mut HashMap::new())
    }

    fn eval(&self, vars: &mut HashMap<Identifier, Value>) -> i64 {
        match self {
            Statement::Assign {
                name,
                value: statement,
                ..
            } => {
                let value = statement.eval(vars);
                vars.insert(name.clone(), value);
                value
            }
            Statement::Operation { lhs, rhs, op } => {
                let l = lhs.eval(vars);
                let r = rhs.eval(vars);
                match op {
                    Operator::Add => l + r,
                    Operator::Sub => l - r,
                    Operator::Mul => l * r,
                    Operator::Div => l / r,
                    Operator::Mod => l % r,
                }
            }
            Statement::Variable(name) => vars[name],
            Statement::Litteral(l) => *l,
            Statement::Sequence(l, r) => {
                l.eval(vars);
                r.eval(vars)
            }
            Statement::If {
                cond,
                case_true,
                case_false,
            } => {
                if cond.eval(vars) != 0 {
                    case_true.eval(vars)
                } else {
                    case_false.eval(vars)
                }
            }
            Statement::While { cond, body } => {
                while cond.eval(vars) != 0 {
                    body.eval(vars);
                }
                0
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::Operator::*;
    use crate::*;

    #[test]
    fn add() {
        // 1 + 1
        assert_eq!(op(Add, val(1), val(1)).eval_empty(), 2);
    }

    #[test]
    fn sub() {
        // 2 - 4
        assert_eq!(op(Sub, val(2), val(4)).eval_empty(), -2);
    }

    #[test]
    fn mul() {
        // 4 * 3
        assert_eq!(op(Mul, val(4), val(3)).eval_empty(), 12);
    }

    #[test]
    fn div() {
        // 7 / 2
        assert_eq!(op(Div, val(7), val(2)).eval_empty(), 3);
    }

    #[test]
    fn modulo() {
        // 8 % 3
        assert_eq!(op(Mod, val(8), val(3)).eval_empty(), 2);
    }

    #[test]
    fn complex_1() {
        // (2 + 3) * 5
        assert_eq!(op(Mul, op(Add, val(2), val(3)), val(5)).eval_empty(), 25);
    }

    #[test]
    fn complex_2() {
        // ((3 + 4) - ((5 * 3) % 2)
        assert_eq!(
            op(
                Sub,
                op(Add, val(3), val(4)),
                op(Mod, op(Mul, val(5), val(3)), val(2))
            )
            .eval_empty(),
            6
        );
    }

    #[test]
    fn assign_eval() {
        // let x = 4
        // x
        assert_eq!(s(a("n", val(4)), var("n")).eval_empty(), 4);
    }

    #[test]
    fn sequence() {
        // 2 + 3
        // 4 - 2
        assert_eq!(
            s(op(Add, val(2), val(3)), op(Sub, val(4), val(2))).eval_empty(),
            2
        );
    }

    #[test]
    fn conditional() {
        // if 1 {
        //   2
        // } else {
        //   3
        // }
        assert_eq!(fi(val(1), val(2), val(3)).eval_empty(), 2);
    }

    #[test]
    fn factorial() {
        // n = 5
        // f = 1
        //
        //
        // while (n - 1) {
        //   n = n - 1
        //   f = f * n
        // }
        // b

        assert_eq!(
            s(
                s(
                    s(a("n", val(5)), a("f", val(1))),
                    w(
                        op(Sub, var("n"), val(1)),
                        s(
                            a("f", op(Mul, var("f"), var("n"))),
                            a("n", op(Sub, var("n"), val(1))),
                        )
                    )
                ),
                var("f")
            )
            .eval_empty(),
            120
        );
    }
}

// ------------------------------------
// Useful definitions

fn val(val: i64) -> Statement {
    Statement::Litteral(val)
}

fn var(name: &str) -> Statement {
    Statement::Variable(name.to_owned())
}

fn op(o: Operator, l: Statement, r: Statement) -> Statement {
    Statement::Operation {
        lhs: Box::new(l),
        rhs: Box::new(r),
        op: o,
    }
}

fn a(name: &str, value: Statement) -> Statement {
    Statement::Assign {
        name: name.to_owned(),
        value: Box::new(value),
    }
}

fn s(l: Statement, r: Statement) -> Statement {
    Statement::Sequence(Box::new(l), Box::new(r))
}

fn fi(cond: Statement, t: Statement, f: Statement) -> Statement {
    Statement::If {
        cond: Box::new(cond),
        case_true: Box::new(t),
        case_false: Box::new(f),
    }
}

fn w(cond: Statement, body: Statement) -> Statement {
    Statement::While {
        cond: Box::new(cond),
        body: Box::new(body),
    }
}
