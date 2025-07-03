use crate::{
    criteria::Capacity,
    insert::{
        criteria::capacity::move_generator::CapacityMoveGenerator, neighborhood::InsertNeighborhood,
    },
};
use orx_local_search::CriterionWithNeighborhood;

#[derive(Default, Clone, Copy)]
pub struct CapacityInsert;

impl CriterionWithNeighborhood for CapacityInsert {
    type Criterion = Capacity;

    type Neighborhood = InsertNeighborhood;

    type MoveGenerator<'i> = CapacityMoveGenerator;

    fn move_generator<'i>(self) -> Self::MoveGenerator<'i> {
        CapacityMoveGenerator
    }
}
