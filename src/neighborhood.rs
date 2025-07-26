use crate::r#move::Move;

pub trait Neighborhood: 'static + Default + Clone + Copy {
    type Move: Move;

    fn neighborhood(solution: &<Self::Move as Move>::Solution) -> impl Iterator<Item = Self::Move>;
}
