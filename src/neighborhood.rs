use crate::{Problem, r#move::Move};

pub trait Neighborhood: 'static + Default + Clone + Copy {
    type Problem: Problem;

    type Move: Move<Solution = <Self::Problem as Problem>::Solution>;

    fn neighborhood<'a, 'b, 'c>(
        solution: &'b <Self::Move as Move>::Solution,
    ) -> impl Iterator<Item = Self::Move> + 'a + 'b + 'c;
}
