use std::sync::{Arc, Mutex};

use chumsky::prelude::*;

#[derive(Debug)]
enum Expr<'a> {
    Num(f64),
    Var(&'a str),

    Neg(Box<Expr<'a>>),
    Add(Box<Expr<'a>>, Box<Expr<'a>>),
    Sub(Box<Expr<'a>>, Box<Expr<'a>>),
    Mul(Box<Expr<'a>>, Box<Expr<'a>>),
    Div(Box<Expr<'a>>, Box<Expr<'a>>),
    Pow(Box<Expr<'a>>, Box<Expr<'a>>),

    Let {
        name: &'a str,
        rhs: Box<Expr<'a>>,
        then: Box<Expr<'a>>,
    },
}

pub struct Expiration<'a> {
    ast: Expr<'a>,
}

impl<'a> Expiration<'a> {
    pub fn new(input: &'a str) -> Result<Self, String> {
        match Self::parse().parse(&input).into_result() {
            Ok(ast) => Ok(Self { ast }),
            Err(parse_errs) => {
                let mut er = String::new();
                parse_errs
                    .into_iter()
                    .for_each(|e| er = format!("{er}\nParse error: {e}"));
                Err(er)
            }
        }
    }

    fn parse() -> impl Parser<'a, &'a str, Expr<'a>> {
        let ident = text::ascii::ident().padded();

        let expr = recursive(|expr| {
            let int = text::int(10).map(|s: &str| Expr::Num(s.parse().unwrap()));

            let atom = int
                .or(expr.delimited_by(just('('), just(')')))
                .or(ident.map(Expr::Var))
                .padded();

            let op = |c| just(c).padded();

            let unary = op('-')
                .repeated()
                .foldr(atom, |_op, rhs| Expr::Neg(Box::new(rhs)));

            let power = unary.clone().foldl(
                choice((op('^').to(Expr::Mul as fn(_, _) -> _),))
                    .then(unary)
                    .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            );

            let product = power.clone().foldl(
                choice((
                    op('*').to(Expr::Mul as fn(_, _) -> _),
                    op('/').to(Expr::Div as fn(_, _) -> _),
                ))
                .then(power)
                .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            );

            let sum = product.clone().foldl(
                choice((
                    op('+').to(Expr::Add as fn(_, _) -> _),
                    op('-').to(Expr::Sub as fn(_, _) -> _),
                ))
                .then(product)
                .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            );

            sum
        });

        let decl = recursive(|decl| {
            let r#let = text::ascii::keyword("let")
                .ignore_then(ident)
                .then_ignore(just('='))
                .then(expr.clone())
                .then_ignore(just(';'))
                .then(decl.clone())
                .map(|((name, rhs), then)| Expr::Let {
                    name,
                    rhs: Box::new(rhs),
                    then: Box::new(then),
                });

            r#let.or(expr).padded()
        });

        decl
    }

    pub fn calculate(&'a self) -> Result<f64, String> {
        Self::eval(&self.ast, Arc::new(Mutex::new(Vec::new())))
    }

    fn binary_operator(
        l: &'a Expr<'a>,
        r: &'a Expr<'a>,
        vars: Arc<Mutex<Vec<(&'a str, f64)>>>,
        f: fn(f64, f64) -> f64,
    ) -> Result<f64, String> {
        let (left_result, right_result) = rayon::join(
            || Self::eval(l, vars.clone()),
            || Self::eval(r, vars.clone()),
        );

        let left = left_result?;
        let right = right_result?;

        Ok(f(left, right))
    }

    fn eval(expr: &'a Expr<'a>, vars: Arc<Mutex<Vec<(&'a str, f64)>>>) -> Result<f64, String> {
        match expr {
            Expr::Num(x) => Ok(*x),
            Expr::Neg(a) => Ok(-Self::eval(a, vars)?),
            Expr::Add(a, b) => Self::binary_operator(&a, &b, vars, |x, y| x + y),
            Expr::Sub(a, b) => Self::binary_operator(&a, &b, vars, |x, y| x - y),
            Expr::Mul(a, b) => Self::binary_operator(&a, &b, vars, |x, y| x * y),
            Expr::Div(a, b) => Self::binary_operator(&a, &b, vars, |x, y| x / y),
            Expr::Pow(a, b) => Self::binary_operator(&a, &b, vars, |x, y| x.powf(y)),
            Expr::Var(name) => {
                let vars = vars.lock().unwrap();
                if let Some((_, val)) = vars.iter().rev().find(|(var, _)| var == name) {
                    Ok(*val)
                } else {
                    Err(format!("Cannot find variable `{}` in scope", name))
                }
            }
            Expr::Let { name, rhs, then } => {
                let rhs_value = Self::eval(rhs, vars.clone())?;

                {
                    let mut vars = vars.lock().unwrap();
                    vars.push((*name, rhs_value));
                }
                let output = Self::eval(then, vars.clone());

                {
                    let mut vars = vars.lock().unwrap();
                    vars.pop();
                }

                output
            }
        }
    }
}
