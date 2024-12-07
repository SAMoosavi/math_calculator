use std::{
    cmp::max,
    fmt::{Display, Formatter},
};

pub struct Var {
    var: String,
}

impl Display for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.var)
    }
}

pub struct Val {
    val: i32,
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

pub struct BinaryOperator {
    left: Box<Element>,
    right: Box<Element>,
    depth: u32,
}

pub enum Element {
    Var(Var),
    Val(Val),
    Add(BinaryOperator),
    Subtract(BinaryOperator),
    Multiple(BinaryOperator),
    Division(BinaryOperator),
    Power(BinaryOperator),
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Var(x) => write!(f, "{}", x),
            Element::Val(x) => write!(f, "{}", x),
            Element::Add(x) => write!(f, "({} + {})", x.left, x.right),
            Element::Subtract(x) => write!(f, "({} - {})", x.left, x.right),
            Element::Multiple(x) => write!(f, "({} * {})", x.left, x.right),
            Element::Division(x) => write!(f, "({} / {})", x.left, x.right),
            Element::Power(x) => write!(f, "({} ^ {})", x.left, x.right),
        }
    }
}

impl Element {
    pub fn from_operator(left: Element, operator: &str, right: Element) -> Self {
        let depth = max(left.get_depth(), right.get_depth()) + 1;
        let left = Box::new(left);
        let right = Box::new(right);
        let binary_operator = BinaryOperator { left, right, depth };
        match operator {
            "+" => Element::Add(binary_operator),
            "-" => Element::Subtract(binary_operator),
            "*" => Element::Multiple(binary_operator),
            "/" => Element::Division(binary_operator),
            "^" => Element::Power(binary_operator),
            _ => panic!("Unknown Element"),
        }
    }

    pub fn from_var(var: String) -> Self {
        Self::Var(Var { var })
    }

    pub fn from_val(val: i32) -> Self {
        Self::Val(Val { val })
    }

    fn get_depth(&self) -> u32 {
        match self {
            Element::Val(_) => 0,
            Element::Var(_) => 0,
            Element::Add(x) => x.depth,
            Element::Subtract(x) => x.depth,
            Element::Multiple(x) => x.depth,
            Element::Division(x) => x.depth,
            Element::Power(x) => x.depth,
        }
    }
}
