use crate::criteria::duration::{criterion::Duration, iter::DurationMoves};
use orx_local_search::{CandidateMoveOf, InputOf, MoveGenerator, SolutionOf};

pub struct DurationMoveGenerator;

impl MoveGenerator for DurationMoveGenerator {
    type X = Duration;

    fn moves<'a, 's, 'i>(
        &'a mut self,
        tour: &'s SolutionOf<Self::X>,
        duration_matrix: InputOf<'i, Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Self::X>> + 'a + 's + 'i
    where
        'i: 's,
        's: 'i,
        'a: 's,
    {
        DurationMoves::new(self, tour, duration_matrix)
    }
}
