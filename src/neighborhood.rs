use crate::{r#move::Move, problem::Problem};

pub trait Neighborhood: 'static + Default + Clone + Copy {
    type Problem: Problem;

    type Move: Move<Problem = Self::Problem>;

    fn neighborhood(
        solution: &<Self::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = Self::Move>;
}
