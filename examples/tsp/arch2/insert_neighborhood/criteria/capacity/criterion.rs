use crate::{
    criteria::Capacity,
    insert_neighborhood::{
        criteria::capacity::move_generator::CapacityMoveGenerator, neighborhood::InsertNeighborhood,
    },
    tsp::Tsp,
};
use orx_local_search::CriterionOnNeighborhood;

pub struct CapacityOnInsert;

impl CriterionOnNeighborhood for CapacityOnInsert {
    type Problem = Tsp;

    type Criterion = Capacity;

    type Neighborhood = InsertNeighborhood;

    type MoveGenerator<'i> = CapacityMoveGenerator;
}
