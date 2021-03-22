use crate::Grid;
use math::{Int2, IntRectIter};

pub struct GridIter<'a, T> {
    pub(crate) grid: &'a Grid<T>,
    pub(crate) iter: IntRectIter,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (Int2, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .and_then(|pos| self.grid.getp(pos).and_then(|val| Some((pos, val))))
    }
}
