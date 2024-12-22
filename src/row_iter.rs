use crate::{Grid, GridMut, Row};
use std::iter::FusedIterator;

#[derive(Clone)]
pub struct RowIter<GridRef> {
    pub(crate) grid: GridRef,
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) r: usize,
}

impl<GridRef> RowIter<GridRef> {
    #[inline]
    pub(crate) fn new(grid: GridRef, y: usize, r: usize) -> RowIter<GridRef> {
        Self { grid, x: 0, y, r }
    }
}

// ---------- IMMUTABLE ITERATOR ----------

impl<'a, G: Grid> Iterator for RowIter<&'a G> {
    type Item = &'a G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.r {
            let x = self.x;
            self.x += 1;
            self.grid.get(x, self.y)
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
        if self.x < self.r {
            self.grid.get(self.r - 1, self.y)
        } else {
            None
        }
    }
}

impl<'a, G: Grid> DoubleEndedIterator for RowIter<&'a G> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x < self.r {
            self.r -= 1;
            self.grid.get(self.r, self.y)
        } else {
            None
        }
    }
}

impl<'a, G: Grid> ExactSizeIterator for RowIter<&'a G> {
    #[inline]
    fn len(&self) -> usize {
        self.r - self.x
    }
}

impl<'a, G: Grid> FusedIterator for RowIter<&'a G> {}

// ---------- MUTABLE ITERATOR ----------

impl<'a, G: GridMut> Iterator for RowIter<&'a mut G> {
    type Item = &'a mut G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.r {
            let item: *mut G::Item = self.grid.get_mut(self.x, self.y).unwrap();
            self.x += 1;
            Some(unsafe { &mut *item })
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
        if self.x < self.r {
            self.grid.get_mut(self.r - 1, self.y)
        } else {
            None
        }
    }
}

impl<'a, G: GridMut> DoubleEndedIterator for RowIter<&'a mut G> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x < self.r {
            let item: *mut G::Item = self.grid.get_mut(self.r, self.y).unwrap();
            self.r -= 1;
            Some(unsafe { &mut *item })
        } else {
            None
        }
    }
}

impl<'a, G: GridMut> ExactSizeIterator for RowIter<&'a mut G> {
    #[inline]
    fn len(&self) -> usize {
        self.r - self.x
    }
}

impl<'a, G: GridMut> FusedIterator for RowIter<&'a mut G> {}
