use std::fmt::{Debug, Display, Formatter};
use crate::parser::{Element, Types};

pub struct Power {
    left: Types,
    right: Types,
}

impl Display for Power {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} ^ {})", self.left, self.right)
    }
}

impl Element for Power {
    fn new(left: Types, right: Types) -> Self
    where
        Self: Sized,
    {
        Self { left, right }
    }

}
