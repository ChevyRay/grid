use crate::Grid;
use math::IntRectIter;

pub struct GridValuesMut<'a, T> {
    pub(crate) grid: &'a mut Grid<T>,
    pub(crate) iter: IntRectIter,
}

impl<'a, T> Iterator for GridValuesMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|pos| {
            self.grid.get_mut(pos.x, pos.y).and_then(|val| {
                let ptr: *mut T = val;
                unsafe { Some(&mut *ptr) }
            })
        })
    }
}
