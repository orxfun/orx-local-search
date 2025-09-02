use crate::tsp::Tsp;
use orx_iterable::Collection;
use orx_local_search::{Neighborhood, Problem};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct InsertMove {
    pub current_position: usize,
    pub target_position: usize,
}

#[derive(Default, Clone, Copy)]
pub struct Insert;

impl Neighborhood<Tsp> for Insert {
    type Move = InsertMove;

    fn neighborhood(tour: &<Tsp as Problem>::Solution) -> impl Iterator<Item = Self::Move> {
        AllInsertMovesIter::new(tour.iter().len())
    }

    fn apply_move(mv: &Self::Move, tour: &mut <Tsp as Problem>::Solution) {
        match mv.current_position.cmp(&mv.target_position) {
            Ordering::Equal => {}
            Ordering::Less => {
                let current = tour[mv.current_position];
                for p in mv.current_position..mv.target_position {
                    tour[p] = tour[p + 1];
                }
                tour[mv.target_position] = current;
            }
            Ordering::Greater => {
                let current = tour[mv.current_position];
                for p in (mv.target_position..mv.current_position).rev() {
                    tour[p + 1] = tour[p];
                }
                tour[mv.target_position] = current;
            }
        }
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
                let mv = InsertMove {
                    current_position: self.current_position,
                    target_position: self.target_position,
                };
                self.target_position += 1;
                Some(mv)
            }
            true => self.next_current_position(),
        }
    }
}
