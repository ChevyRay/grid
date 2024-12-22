use crate::{Grid, GridMut, Row};
use std::iter::FusedIterator;

#[derive(Clone)]
pub struct Rows<GridRef> {
    pub(crate) grid: GridRef,
    pub(crate) y: usize,
    pub(crate) h: usize,
}

impl<'a, G: Grid> Iterator for Rows<&'a G> {
    type Item = Row<&'a G>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.h {
            let row = Row {
                grid: self.grid,
                y: self.y,
            };
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
        self.h.checked_sub(1).map(|y| Row { grid: self.grid, y })
    }
}

impl<'a, G: Grid> DoubleEndedIterator for Rows<&'a G> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.h > self.y {
            self.h -= 1;
            Some(Row {
                grid: self.grid,
                y: self.h,
            })
        } else {
            None
        }
    }
}

impl<'a, G: Grid> ExactSizeIterator for Rows<&'a G> {
    #[inline]
    fn len(&self) -> usize {
        self.h - self.y
    }
}

impl<'a, G: Grid> FusedIterator for Rows<&'a G> {}

impl<'a, G: GridMut> Iterator for Rows<&'a mut G> {
    type Item = Row<&'a mut G>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.h {
            let grid: *mut G = self.grid;
            let row = Row {
                grid: unsafe { &mut *grid },
                y: self.y,
            };
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
        self.h.checked_sub(1).map(|y| Row { grid: self.grid, y })
    }
}

impl<'a, G: GridMut> DoubleEndedIterator for Rows<&'a mut G> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.h > self.y {
            self.h -= 1;
            let grid: *mut G = self.grid;
            Some(Row {
                grid: unsafe { &mut *grid },
                y: self.h,
            })
        } else {
            None
        }
    }
}

impl<'a, G: GridMut> ExactSizeIterator for Rows<&'a mut G> {
    #[inline]
    fn len(&self) -> usize {
        self.h - self.y
    }
}

impl<'a, G: GridMut> FusedIterator for Rows<&'a mut G> {}
