use crate::{insert_neighborhood::r#move::Insert, tsp::Tsp};
use orx_iterable::Collection;
use orx_local_search::{Neighborhood, Problem};

#[derive(Default, Clone, Copy)]
pub struct InsertNeighborhood;

impl Neighborhood for InsertNeighborhood {
    type Problem = Tsp;

    type Move = Insert;

    fn neighborhood(
        tour: &<Self::Problem as Problem>::Solution,
    ) -> impl Iterator<Item = Self::Move> {
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

    fn next_current_position(&mut self) -> Option<Insert> {
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
    type Item = Insert;

    fn next(&mut self) -> Option<Self::Item> {
        match self.target_position == self.n {
            false => {
                let mv = Insert::new(self.current_position, self.target_position);
                self.target_position += 1;
                Some(mv)
            }
            true => self.next_current_position(),
        }
    }
}
