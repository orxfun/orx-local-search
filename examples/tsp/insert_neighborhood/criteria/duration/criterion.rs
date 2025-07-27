use crate::{
    criteria::Duration,
    insert_neighborhood::{
        criteria::duration::move_generator::DurationMoveGenerator, neighborhood::InsertNeighborhood,
    },
    tsp::Tsp,
};
use orx_local_search::CriterionOnNeighborhood;

pub struct DurationOnInsert;

impl CriterionOnNeighborhood for DurationOnInsert {
    type Problem = Tsp;

    type Criterion = Duration;

    type Neighborhood = InsertNeighborhood;

    type MoveGenerator<'i> = DurationMoveGenerator;
}
