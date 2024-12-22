use crate::{Grid, GridMut, Row};
use std::iter::FusedIterator;

#[derive(Clone)]
pub struct Rows<GridRef> {
    grid: GridRef,
    y: usize,
    b: usize,
}

impl<GridRef> Rows<GridRef> {
    #[inline]
    pub(crate) fn new(grid: GridRef, b: usize) -> Self {
        Self { grid, y: 0, b }
    }
}

impl<'a, G: Grid> Iterator for Rows<&'a G> {
    type Item = Row<&'a G>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.b {
            let row = Row::new(self.grid, self.y);
            self.y += 1;
            Some(row)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        if self.y < self.b {
            Some(Row::new(self.grid, self.b - 1))
        } else {
            None
        }
    }
}

impl<'a, G: Grid> DoubleEndedIterator for Rows<&'a G> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.y < self.b {
            self.b -= 1;
            Some(Row::new(self.grid, self.b))
        } else {
            None
        }
    }
}

impl<'a, G: Grid> ExactSizeIterator for Rows<&'a G> {
    #[inline]
    fn len(&self) -> usize {
        self.b - self.y
    }
}

impl<'a, G: Grid> FusedIterator for Rows<&'a G> {}

impl<'a, G: GridMut> Iterator for Rows<&'a mut G> {
    type Item = Row<&'a mut G>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.b {
            let grid: *mut G = self.grid;
            let row = Row::new(unsafe { &mut *grid }, self.y);
            self.y += 1;
            Some(row)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }

    #[inline]
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        if self.y < self.b {
            let grid: *mut G = self.grid;
            Some(Row::new(unsafe { &mut *grid }, self.b - 1))
        } else {
            None
        }
    }
}

impl<'a, G: GridMut> DoubleEndedIterator for Rows<&'a mut G> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.y < self.b {
            self.b -= 1;
            let grid: *mut G = self.grid;
            Some(Row::new(unsafe { &mut *grid }, self.b))
        } else {
            None
        }
    }
}

impl<'a, G: GridMut> ExactSizeIterator for Rows<&'a mut G> {
    #[inline]
    fn len(&self) -> usize {
        self.b - self.y
    }
}

impl<'a, G: GridMut> FusedIterator for Rows<&'a mut G> {}
