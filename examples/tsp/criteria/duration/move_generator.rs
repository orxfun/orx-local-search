use crate::criteria::duration::{criterion::Duration, iter::DurationMoves};
use orx_local_search::{CandidateMoveOf, InputOf, MoveGenerator, SolutionOf};

pub struct DurationMoveGenerator;

impl MoveGenerator for DurationMoveGenerator {
    type X = Duration;

    fn moves<'a, 'b, 'c>(
        &'a mut self,
        tour: &'b SolutionOf<Self::X>,
        duration_matrix: InputOf<'c, Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::X>> + 'a + 'b + 'c {
        DurationMoves::new(tour, duration_matrix)
    }
}
