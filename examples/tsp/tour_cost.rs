use orx_local_search::ObjectiveValue;

pub struct TourCost(u64);

impl ObjectiveValue for TourCost {
    type Unit = u64;

    fn identity() -> Self::Unit {
        0
    }

    fn reduce(a: Self::Unit, b: Self::Unit) -> Self::Unit {
        a + b
    }
}
