use crate::{
    criteria::capacity::{CapacityInput, iter::CapacityMoves},
    problem::Tsp,
};
use orx_local_search::{CandidateMoveOf, MoveGenerator, Problem};

pub struct CapacityMoveGenerator;

impl<'i> MoveGenerator<'i> for CapacityMoveGenerator {
    type Problem = Tsp;

    type Input = CapacityInput;

    fn moves<'a>(
        &'a mut self,
        tour: &'a <Self::Problem as Problem>::Solution,
        input: &'a Self::Input,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::Problem>> + 'a {
        CapacityMoves::new(tour, input)
    }
}
