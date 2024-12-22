use crate::{Grid, GridMut, Row};
use std::iter::FusedIterator;

#[derive(Clone)]
pub struct RowsIter<GridRef> {
    grid: GridRef,
    y: usize,
    b: usize,
}

impl<GridRef> RowsIter<GridRef> {
    #[inline]
    pub(crate) fn new(grid: GridRef, b: usize) -> Self {
        Self { grid, y: 0, b }
    }
}

impl<'a, G: Grid> Iterator for RowsIter<&'a G> {
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

impl<G: Grid> DoubleEndedIterator for RowsIter<&G> {
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

impl<G: Grid> ExactSizeIterator for RowsIter<&G> {
    #[inline]
    fn len(&self) -> usize {
        self.b - self.y
    }
}

impl<G: Grid> FusedIterator for RowsIter<&G> {}

impl<'a, G: GridMut> Iterator for RowsIter<&'a mut G> {
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

impl<G: GridMut> DoubleEndedIterator for RowsIter<&mut G> {
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

impl<G: GridMut> ExactSizeIterator for RowsIter<&mut G> {
    #[inline]
    fn len(&self) -> usize {
        self.b - self.y
    }
}

impl<G: GridMut> FusedIterator for RowsIter<&mut G> {}
