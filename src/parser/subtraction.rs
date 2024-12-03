use crate::parser::{Element, Types};
use std::fmt::{Display, Formatter};

pub struct Subtraction {
    left: Types,
    right: Types,
}

impl Display for Subtraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} - {})", self.left, self.right)
    }
}

impl Element for Subtraction {
    fn new(left: Types, right: Types) -> Self {
        Self { left, right }
    }
}
