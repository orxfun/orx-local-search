use crate::{tour::Tour, tsp::Tsp};
use orx_iterable::Collection;
use orx_local_search::{Criterion, Problem};

#[derive(Default, Clone, Copy)]
pub struct Duration;

impl Criterion for Duration {
    type Problem = Tsp;

    type Input<'i> = DurationMatrix;

    fn evaluate(
        self,
        input: &Self::Input<'_>,
        tour: &<Self::Problem as Problem>::Solution,
    ) -> Option<<Self::Problem as Problem>::ObjectiveUnit> {
        Some(input.tour_duration(tour))
    }
}

pub struct DurationMatrix(Vec<Vec<u64>>);

impl DurationMatrix {
    pub fn new(matrix: Vec<Vec<u64>>) -> Self {
        Self(matrix)
    }

    pub fn get(&self, from_city: usize, to_city: usize) -> u64 {
        self.0[from_city][to_city]
    }

    pub fn tour_len(&self) -> usize {
        self.0.len()
    }

    pub fn tour_duration(&self, tour: &Tour) -> u64 {
        let mut cost = 0;
        let a = tour.iter().copied();
        let b = tour.iter().copied().skip(1);
        for (a, b) in a.zip(b) {
            cost += self.get(a, b);
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
