use crate::criteria::duration::{criterion::Duration, iter::DurationMoves};
use orx_local_search::{CandidateMoveOf, InputOf, MoveGenerator, SolutionOf};

pub struct DurationMoveGenerator;

impl MoveGenerator for DurationMoveGenerator {
    type X = Duration;

    fn moves<'a>(
        &mut self,
        tour: &'a SolutionOf<Self::X>,
        duration_matrix: &'a InputOf<'a, Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::X>> {
        DurationMoves::new(tour, duration_matrix)
    }
}
