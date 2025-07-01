use crate::{
    criteria::capacity::{criterion::Capacity, iter::CapacityMoves},
    problem::Tsp,
};
use orx_local_search::{CandidateMoveOf, InputOf, MoveGenerator, SolutionOf};

pub struct CapacityMoveGenerator;

impl MoveGenerator for CapacityMoveGenerator {
    type X = Capacity;

    fn moves<'a>(
        &'a mut self,
        tour: &'a SolutionOf<Self::X>,
        duration_matrix: &'a InputOf<Self::X>,
    ) -> impl Iterator<Item = CandidateMoveOf<Tsp>> + 'a {
        CapacityMoves::new(tour, duration_matrix)
    }
}
