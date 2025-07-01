use crate::{
    criteria::duration::{criterion::Duration, iter::DurationMoves},
    problem::Tsp,
};
use orx_local_search::{CandidateMoveOf, InputOf, MoveGenerator, SolutionOf};

pub struct DurationMoveGenerator;

impl MoveGenerator for DurationMoveGenerator {
    type X = Duration;

    fn moves<'a>(
        &'a mut self,
        tour: &'a SolutionOf<Self::X>,
        duration_matrix: &'a InputOf<Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Tsp>> + 'a {
        DurationMoves::new(tour, duration_matrix)
    }
}
