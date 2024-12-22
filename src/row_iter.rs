use crate::{Grid, GridMut, Row};
use std::iter::FusedIterator;

#[derive(Clone)]
pub struct RowIter<RowRef> {
    pub(crate) row: RowRef,
    pub(crate) x: usize,
    pub(crate) w: usize,
}

// ---------- RowIter<Row<&'a G>> ----------

impl<'a, G: Grid> Iterator for RowIter<Row<&'a G>> {
    type Item = &'a G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.w {
            let val: *const G::Item = self.row.get(self.x).unwrap();
            self.x += 1;
            Some(unsafe { &*val })
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
        let x = self.w.checked_sub(1)?;
        self.row.grid.get(x, self.row.y)
    }
}

impl<'a, G: Grid> ExactSizeIterator for RowIter<Row<&'a G>> {
    #[inline]
    fn len(&self) -> usize {
        self.w - self.x
    }
}

impl<'a, G: Grid> FusedIterator for RowIter<Row<&'a G>> {}

impl<'a, G: Grid> DoubleEndedIterator for RowIter<Row<&'a G>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x > self.w {
            self.w -= 1;
            self.row.grid.get(self.w, self.row.y)
        } else {
            None
        }
    }
}

// ---------- RowIter<&'a Row<&'a G>> ----------

impl<'a, G: Grid> Iterator for RowIter<&'a Row<&'a G>> {
    type Item = &'a G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.w {
            let val = self.row.get(self.x).unwrap();
            self.x += 1;
            Some(val)
        } else {
            None
        }
    }
}

// ---------- RowIter<&'a Row<&'a mut G>> ----------

impl<'a, G: Grid> Iterator for RowIter<&'a Row<&'a mut G>> {
    type Item = &'a G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.w {
            let val = self.row.get(self.x).unwrap();
            self.x += 1;
            Some(val)
        } else {
            None
        }
    }
}

// ---------- RowIter<&'a mut Row<&'a mut G>> ----------

impl<'a, G: GridMut> Iterator for RowIter<&'a mut Row<&'a mut G>> {
    type Item = &'a mut G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let val: *mut G::Item = self.row.get_mut(self.x)?;
        self.x += 1;
        Some(unsafe { &mut *val })
    }
}
