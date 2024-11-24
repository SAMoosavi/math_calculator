use crate::parser::{Element, Types};

pub struct Subtraction {
    left: Types,
    right: Types,
}

impl Element for Subtraction {
    fn new(left: Types, right: Types) -> Self
    {
        Self { left, right }
    }
}
