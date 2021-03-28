use crate::{Grid, GridIndex};
use math::{Int2, IntRect, IntRectIter};
use std::marker::PhantomData;

pub struct ConstGrid<T, const W: usize, const H: usize> {
    data: [[T; W]; H],
}

impl<T: Default + Copy, const W: usize, const H: usize> Default for ConstGrid<T, W, H> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Copy, const W: usize, const H: usize> ConstGrid<T, W, H> {
    pub fn new<U: Into<T>>(fill: U) -> Self {
        Self {
            data: [[fill.into(); W]; H],
        }
    }
}

impl<T, const W: usize, const H: usize> Grid<T> for ConstGrid<T, W, H> {
    fn width(&self) -> usize {
        W
    }

    fn height(&self) -> usize {
        H
    }

    fn get<I: GridIndex<T>>(&self, index: I) -> Option<&T> {
        let (x, y) = index.pos(W);
        self.data.get(y).and_then(|data| data.get(x))
    }

    fn get_mut<I: GridIndex<T>>(&mut self, index: I) -> Option<&mut T> {
        let (x, y) = index.pos(W);
        self.data.get_mut(y).and_then(|data| data.get_mut(x))
    }
}

impl<T, const W: usize, const H: usize> ConstGrid<T, W, H> {
    pub unsafe fn get_unchecked<I: GridIndex<T>>(&self, index: I) -> &T {
        let (x, y) = index.pos(W);
        self.data.get_unchecked(y).get_unchecked(x)
    }

    pub unsafe fn get_unchecked_mut<I: GridIndex<T>>(&mut self, index: I) -> &mut T {
        let (x, y) = index.pos(W);
        self.data.get_unchecked_mut(y).get_unchecked_mut(x)
    }

    pub unsafe fn set_unchecked<I: GridIndex<T>, U: Into<T>>(&mut self, index: I, value: U) {
        *self.get_unchecked_mut(index) = value.into();
    }

    pub fn in_rect_unchecked(&self, rect: IntRect) -> ConstGridIter<T, W, H> {
        let rect = self
            .bounds()
            .overlap(&rect)
            .unwrap_or_else(|| IntRect::EMPTY);
        ConstGridIter {
            grid: self,
            iter: rect.iter(),
            _marker: PhantomData::default(),
        }
    }

    pub fn in_rect_unchecked_mut(&mut self, rect: IntRect) -> ConstGridIterMut<T, W, H> {
        let rect = self
            .bounds()
            .overlap(&rect)
            .unwrap_or_else(|| IntRect::EMPTY);
        ConstGridIterMut {
            grid: self,
            iter: rect.iter(),
            _marker: PhantomData::default(),
        }
    }

    pub fn iter_unchecked(&self) -> ConstGridIter<T, W, H> {
        self.in_rect_unchecked(self.bounds())
    }

    pub fn iter_unchecked_mut(&mut self) -> ConstGridIterMut<T, W, H> {
        self.in_rect_unchecked_mut(self.bounds())
    }
}

pub struct ConstGridIter<'a, T, const W: usize, const H: usize> {
    grid: &'a ConstGrid<T, W, H>,
    iter: IntRectIter,
    _marker: PhantomData<T>,
}

impl<'a, T: 'a, const W: usize, const H: usize> Iterator for ConstGridIter<'a, T, W, H> {
    type Item = (Int2, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|pos| {
            let val = unsafe { self.grid.get_unchecked(pos) };
            Some((pos, val))
        })
    }
}

pub struct ConstGridIterMut<'a, T, const W: usize, const H: usize> {
    grid: &'a mut ConstGrid<T, W, H>,
    iter: IntRectIter,
    _marker: PhantomData<T>,
}

impl<'a, T: 'a, const W: usize, const H: usize> Iterator for ConstGridIterMut<'a, T, W, H> {
    type Item = (Int2, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|pos| {
            let val = unsafe { self.grid.get_unchecked_mut(pos) };
            let ptr: *mut T = val;
            unsafe { Some((pos, &mut *ptr)) }
        })
    }
}
