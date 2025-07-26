use crate::r#move::Move;

pub trait Neighborhood: 'static + Default + Clone + Copy {
    type Solution;

    type Move: Move<Solution = Self::Solution>;

    fn neighborhood(solution: &Self::Solution) -> impl Iterator<Item = Self::Move>;
}
