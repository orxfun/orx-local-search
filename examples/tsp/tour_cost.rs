use orx_local_search::Objective;

#[derive(Default, Clone, Copy)]
pub struct TourCost;

impl Objective for TourCost {
    type Unit = u64;

    fn identity() -> Self::Unit {
        0
    }

    fn compose(a: Self::Unit, b: Self::Unit) -> Self::Unit {
        a + b
    }
}
