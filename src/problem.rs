use crate::r#move::Move;

pub trait Problem {
    type On;

    type Value;

    type Move: Move;

    fn all_moves(object: &Self::On) -> impl Iterator<Item = Self::Move>;
}
