use crate::{tour::Tour, tsp::Tsp};
use orx_iterable::Collection;
use orx_local_search::{Criterion, EvalSoln, Problem};

#[derive(Default, Clone, Copy)]
pub struct Duration3;

impl Criterion for Duration3 {
    type Problem = Tsp;

    type Input<'i> = DurationMatrix3;

    fn evaluate(
        duration_matrix: &Self::Input<'_>,
        solution: &<Self::Problem as Problem>::Solution,
    ) -> EvalSoln<Self::Problem> {
        EvalSoln::Feasible(duration_matrix.tour_cost(solution))
    }
}

pub struct DurationMatrix3(Vec<Vec<u64>>);

impl DurationMatrix3 {
    pub fn new(matrix: Vec<Vec<u64>>) -> Self {
        Self(matrix)
    }

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

    pub fn example_input() -> Self {
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
