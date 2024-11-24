use crate::parser::{Element, Types};

pub struct Addition {
    left: Types,
    right: Types,
}

impl Element for Addition {
    fn new(left: Types, right: Types) -> Self
    where
        Self: Sized,
    {
        Self { left, right }
    }
}
