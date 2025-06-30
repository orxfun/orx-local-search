use std::fmt::Debug;

pub trait Move: PartialEq + Eq + Debug {
    type On;

    fn apply(&self, object: Self::On) -> Self::On;
}
