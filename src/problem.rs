use crate::r#move::Move;

pub trait Problem {
    type Object;

    type Value;

    type Move: Move;

    fn neighborhood(object: &Self::Object) -> impl Iterator<Item = Self::Move>;
}
