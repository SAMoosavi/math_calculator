use crate::parser::{Element, Types};
use std::{
    cmp::max,
    fmt::{Display, Formatter},
};

pub struct Power {
    left: Types,
    right: Types,
    depth: u32,
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
        fn calculate_depth(types: &Types) -> u32 {
            match types {
                Types::Element(element) => element.get_depth(),
                _ => 0,
            }
        }

        let depth = max(calculate_depth(&left), calculate_depth(&right)) + 1;

        Self { left, right, depth }
    }

    fn get_depth(&self) -> u32 {
        self.depth
    }
}
