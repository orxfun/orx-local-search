use crate::{tour::Tour, tsp::Tsp};
use orx_iterable::Collection;
use orx_local_search::{Criterion, EvalSoln, Problem};

#[derive(Default, Clone, Copy)]
pub struct Duration;

impl Criterion<Tsp> for Duration {
    type Input<'i> = &'i DurationMatrix;

    fn evaluate(
        &self,
        duration_matrix: &Self::Input<'_>,
        tour: &<Tsp as Problem>::Solution,
    ) -> EvalSoln<Tsp> {
        EvalSoln::Feasible(duration_matrix.tour_cost(tour))
    }
}

pub struct DurationMatrix(Vec<Vec<u64>>);

impl DurationMatrix {
    pub fn get(&self, from_city: usize, to_city: usize) -> u64 {
        self.0[from_city][to_city]
    }

    pub fn tour_len(&self) -> usize {
        self.0.len()
    }

    pub fn tour_cost(&self, tour: &Tour) -> u64 {
        let mut cost = 0;
        for p in 0..tour.iter().len().saturating_sub(1) {
            cost += self.get(tour[p], tour[p + 1]);
        }
        cost
    }

    pub fn example() -> Self {
        Self(vec![
            vec![0, 30, 60, 20, 5, 120],
            vec![20, 0, 10, 50, 80, 70],
            vec![10, 50, 0, 30, 40, 10],
            vec![50, 70, 60, 0, 140, 70],
            vec![10, 80, 30, 50, 0, 20],
            vec![100, 20, 50, 50, 80, 0],
        ])
    }
}
