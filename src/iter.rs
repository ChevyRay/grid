use crate::{Grid, GridMut};
use std::marker::PhantomData;

pub struct Iter<'a, T, G> {
    grid: &'a G,
    x: usize,
    y: usize,
    marker: PhantomData<T>,
}

impl<'a, T, G> Iter<'a, T, G> {
    #[inline]
    pub fn new(grid: &'a G) -> Self {
        Self {
            grid,
            x: 0,
            y: 0,
            marker: PhantomData,
        }
    }
}

impl<'a, T: 'a, G: Grid<T>> Iterator for Iter<'a, T, G> {
    type Item = (&'a T, usize, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.grid.get(self.x, self.y)?;
        let x = self.x;
        let y = self.y;
        self.x += 1;
        if self.x == self.grid.width() {
            self.x = 0;
            self.y += 1;
        }
        Some((val, x, y))
    }
}

pub struct IterMut<'a, T, G> {
    grid: &'a mut G,
    x: usize,
    y: usize,
    marker: PhantomData<T>,
}

impl<'a, T, G> IterMut<'a, T, G> {
    #[inline]
    pub fn new(grid: &'a mut G) -> Self {
        Self {
            grid,
            x: 0,
            y: 0,
            marker: PhantomData,
        }
    }
}

impl<'a, T: 'a, G: GridMut<T>> Iterator for IterMut<'a, T, G> {
    type Item = (&'a mut T, usize, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let val: *mut T = self.grid.get_mut(self.x, self.y)?;
        let x = self.x;
        let y = self.y;
        self.x += 1;
        if self.x == self.grid.width() {
            self.x = 0;
            self.y += 1;
        }
        // SAFETY: this iterator has a mutable reference to the grid,
        // so as long as it exists, the mutable reference to the
        // value fetched from inside the grid will also be valid.
        Some((unsafe { &mut *val }, x, y))
    }
}
