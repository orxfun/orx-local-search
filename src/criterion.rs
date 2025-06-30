use crate::{move_generator::MoveGenerator, problem::Problem};

pub trait Criterion {
    type Problem: Problem;

    type Input<'a>;

    type MoveGenerator: MoveGenerator<X = Self>;
}
