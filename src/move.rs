pub trait Move {
    type On;

    fn apply(&self, object: Self::On) -> Self::On;
}
