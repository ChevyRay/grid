use crate::Grid;
use math::IntRectIter;

pub struct GridValues<'a, T> {
    pub(crate) grid: &'a Grid<T>,
    pub(crate) iter: IntRectIter,
}

impl<'a, T> Iterator for GridValues<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .and_then(|pos| self.grid.get(pos.x, pos.y).and_then(|val| Some(val)))
    }
}
