use crate::{r#move::Move, move_generator::MoveGenerator};

pub trait Criterion {
    type On;

    type Input<'a>;

    type Value;

    type MoveGenerator: MoveGenerator<X = Self>;

    type AllMovesGenerator: MoveGenerator<X = Self>;
}
