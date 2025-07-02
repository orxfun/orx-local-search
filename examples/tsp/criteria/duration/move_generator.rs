use crate::{
    criteria::duration::{DurationMatrix, iter::DurationMoves},
    problem::Tsp,
};
use orx_local_search::{CandidateMoveOf, MoveGenerator, Problem};

pub struct DurationMoveGenerator;

impl<'i> MoveGenerator<'i> for DurationMoveGenerator {
    type Problem = Tsp;

    type Input = DurationMatrix;

    fn moves<'a>(
        &'a mut self,
        tour: &'a <Self::Problem as Problem>::Solution,
        duration_matrix: &'a Self::Input,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::Problem>> + 'a {
        DurationMoves::new(tour, duration_matrix)
    }
}
