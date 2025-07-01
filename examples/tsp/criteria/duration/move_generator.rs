use crate::criteria::duration::{criterion::Duration, iter::DurationMoves};
use orx_local_search::{CandidateMoveOf, InputOf, MoveGenerator, SolutionOf};

pub struct DurationMoveGenerator;

impl MoveGenerator for DurationMoveGenerator {
    type X = Duration;

    fn moves(
        &mut self,
        tour: &SolutionOf<Self::X>,
        duration_matrix: &InputOf<Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::X>> {
        DurationMoves::new(self, tour, duration_matrix)
    }
}
