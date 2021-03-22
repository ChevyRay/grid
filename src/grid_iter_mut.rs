use crate::Grid;
use math::{Int2, IntRectIter};

pub struct GridIterMut<'a, T> {
    pub(crate) grid: &'a mut Grid<T>,
    pub(crate) iter: IntRectIter,
}

impl<'a, T> Iterator for GridIterMut<'a, T> {
    type Item = (Int2, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|pos| {
            self.grid.getp_mut(pos).and_then(|val| {
                let ptr: *mut T = val;
                unsafe { Some((pos, &mut *ptr)) }
            })
        })
    }
}
