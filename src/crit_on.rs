use crate::move_generator::MoveGenerator;

pub trait CriterionOnNeighborhood: Default + Clone + Copy {
    type MoveGenerator<'i>: MoveGenerator;
}
