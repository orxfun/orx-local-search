use crate::{
    criteria::DurationMatrix,
    insert_neighborhood::{
        r#move::Insert,
        neighborhood::{AllInsertMovesIter, InsertNeighborhood},
        tour_after_insert::TourAfterInsertIter,
    },
    tour::Tour,
};
use orx_iterable::Collection;
use orx_local_search::EvalMove;

pub struct DurationMoves<'a> {
    input: &'a DurationMatrix,
    tour: &'a Tour,
    iter: AllInsertMovesIter,
}

impl<'a> DurationMoves<'a> {
    pub fn new(input: &'a DurationMatrix, tour: &'a Tour) -> Self {
        let iter = AllInsertMovesIter::new(tour.iter().len());
        Self { tour, input, iter }
    }

    fn tour_cost_after_move(&self, mv: &Insert) -> u64 {
        let mut cost = 0;
        let mut new_tour = TourAfterInsertIter::new(mv.clone(), self.tour);
        if let Some(mut a) = new_tour.next() {
            for b in new_tour {
                cost += self.input.get(a, b);
                a = b;
            }
        }
        cost
    }
}

impl<'a> Iterator for DurationMoves<'a> {
    type Item = EvalMove<InsertNeighborhood>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|mv| {
            let cost = self.tour_cost_after_move(&mv);
            EvalMove::new(mv, cost)
        })
    }
}
