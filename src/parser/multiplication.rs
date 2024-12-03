use crate::parser::{Element, Types};
use std::fmt::{Display, Formatter};

pub struct Multiplication {
    left: Types,
    right: Types,
}

impl Display for Multiplication {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} * {})", self.left, self.right)
    }
}

impl Element for Multiplication {
    fn new(left: Types, right: Types) -> Self
    where
        Self: Sized,
    {
        Self { left, right }
    }
}
