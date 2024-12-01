use std::fmt::{Debug, Formatter};
use crate::parser::{Element, Types};
use crate::parser::subtraction::Subtraction;

pub struct Addition {
    left: Types,
    right: Types,
}

impl Debug for Addition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.to_string().fmt(f)
    }
}

impl Element for Addition {
    fn new(left: Types, right: Types) -> Self
    where
        Self: Sized,
    {
        Self { left, right }
    }

    fn to_string(&self) -> String {
        let left = match &self.left {
            Types::Var(x) => { x.var.clone() }
            Types::Val(x) => { x.val.to_string() }
            Types::Element(x) => { x.to_string() }
        };

        let right = match &self.right {
            Types::Var(x) => { x.var.clone() }
            Types::Val(x) => { x.val.to_string() }
            Types::Element(x) => { x.to_string() }
        };

        format!("({} + {})", left, right)
    }
}
