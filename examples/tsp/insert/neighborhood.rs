use super::InsertMove;
use crate::tsp::Tsp;
use orx_iterable::Collection;
use orx_local_search::{Move, Neighborhood};

#[derive(Default, Clone, Copy)]
pub struct InsertNeighborhood;

impl Neighborhood for InsertNeighborhood {
    type Problem = Tsp;

    type Move = InsertMove;

    fn neighborhood(tour: &<Self::Move as Move>::Solution) -> impl Iterator<Item = Self::Move> {
        AllInsertMovesIter::new(tour.iter().len())
    }
}

pub struct AllInsertMovesIter {
    n: usize,
    current_position: usize,
    target_position: usize,
}

impl AllInsertMovesIter {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            current_position: 0,
            target_position: 0,
        }
    }

    fn next_current_position(&mut self) -> Option<InsertMove> {
        self.current_position += 1;
        match self.current_position == self.n {
            false => {
                self.target_position = 0;
                self.next()
            }
            true => None,
        }
    }
}

impl Iterator for AllInsertMovesIter {
    type Item = InsertMove;

    fn next(&mut self) -> Option<Self::Item> {
        match self.target_position == self.n {
            false => {
                let mv = InsertMove::new(self.current_position, self.target_position);
                self.target_position += 1;
                Some(mv)
            }
            true => self.next_current_position(),
        }
    }
}
