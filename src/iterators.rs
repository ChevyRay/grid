use crate::Grid;
use math::{Int2, IntRectIter};
use std::marker::PhantomData;

/// Immutable iterator over a region of a grid.
pub struct GridIter<'a, T, G: Grid<T>> {
    pub(crate) grid: &'a G,
    pub(crate) iter: IntRectIter,
    pub(crate) _marker: PhantomData<T>,
}

impl<'a, T: 'a, G: Grid<T>> Iterator for GridIter<'a, T, G> {
    type Item = (Int2, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(pos) = self.iter.next() {
            if let Some(val) = self.grid.get(pos) {
                return Some((pos, val));
            }
        }
        None
    }
}

/// Mutable iterator over a region of a grid.
pub struct GridIterMut<'a, T, G: Grid<T>> {
    pub(crate) grid: &'a mut G,
    pub(crate) iter: IntRectIter,
    pub(crate) _marker: PhantomData<T>,
}

impl<'a, T: 'a, G: Grid<T>> Iterator for GridIterMut<'a, T, G> {
    type Item = (Int2, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(pos) = self.iter.next() {
            if let Some(val) = self.grid.get_mut(pos) {
                let ptr: *mut T = val;
                return unsafe { Some((pos, &mut *ptr)) };
            }
        }
        None
    }
}

/// Unchecked immutable iterator over a region of a grid.
pub struct GridIterUnchecked<'a, T, G: Grid<T>> {
    pub(crate) grid: &'a G,
    pub(crate) iter: IntRectIter,
    pub(crate) _marker: PhantomData<T>,
}

impl<'a, T: 'a, G: Grid<T>> Iterator for GridIterUnchecked<'a, T, G> {
    type Item = (Int2, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .and_then(|pos| Some((pos, unsafe { self.grid.get_unchecked(pos) })))
    }
}

/// Unchecked mutable iterator over a region of a grid.
pub struct GridIterUncheckedMut<'a, T, G: Grid<T>> {
    pub(crate) grid: &'a mut G,
    pub(crate) iter: IntRectIter,
    pub(crate) _marker: PhantomData<T>,
}

impl<'a, T: 'a, G: Grid<T>> Iterator for GridIterUncheckedMut<'a, T, G> {
    type Item = (Int2, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(pos) = self.iter.next() {
            let val = unsafe { self.grid.get_unchecked_mut(pos) };
            let ptr: *mut T = val;
            return unsafe { Some((pos, &mut *ptr)) };
        }
        None
    }
}
