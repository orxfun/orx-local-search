use crate::{insert_move::InsertMove, tour::Tour};
use std::cmp::Ordering;

pub enum TourAfterInsertIter<'a> {
    MoveToRight(MoveToRight<'a>),
    MoveToLeft(MoveToLeft<'a>),
}

impl<'a> TourAfterInsertIter<'a> {
    pub fn new(mv: InsertMove, tour: &'a Tour) -> Self {
        match mv.current_position.cmp(&mv.target_position) {
            // Ordering::Equal => panic!("invalid move"),
            Ordering::Equal | Ordering::Less => Self::MoveToRight(MoveToRight { mv, tour, p: 0 }),
            Ordering::Greater => Self::MoveToLeft(MoveToLeft { mv, tour, p: 0 }),
        }
    }
}

impl Iterator for TourAfterInsertIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::MoveToRight(x) => x.next(),
            Self::MoveToLeft(x) => x.next(),
        }
    }
}

pub struct MoveToRight<'a> {
    mv: InsertMove,
    tour: &'a Tour,
    p: usize,
}

impl Iterator for MoveToRight<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let city = match self.p {
            p if (0..self.mv.current_position).contains(&p) => Some(self.tour[p]),
            p if (self.mv.current_position..self.mv.target_position).contains(&p) => {
                Some(self.tour[p + 1])
            }
            p if p == self.mv.target_position => Some(self.tour[self.mv.current_position]),
            p => self.tour.get(p),
        };
        self.p += 1;
        city
    }
}

pub struct MoveToLeft<'a> {
    mv: InsertMove,
    tour: &'a Tour,
    p: usize,
}

impl Iterator for MoveToLeft<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let city = match self.p {
            p if (0..self.mv.target_position).contains(&p) => Some(self.tour[p]),
            p if p == self.mv.target_position => Some(self.tour[self.mv.current_position]),
            p if ((self.mv.target_position + 1)..=self.mv.current_position).contains(&p) => {
                Some(self.tour[p - 1])
            }
            p => self.tour.get(p),
        };
        self.p += 1;
        city
    }
}
