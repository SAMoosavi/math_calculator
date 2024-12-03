use std::fmt::{Debug, Display, Formatter};
use crate::parser::{Element, Types};

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
    fn new(left: Types, right: Types) -> Self
    {
        Self { left, right }
    }

}
