use crate::{Grid, GridMut, RowIter};
use std::ops::Deref;

/// A single row of a grid.
#[repr(C)]
#[derive(Clone)]
pub struct Row<GridRef> {
    grid: GridRef,
    y: usize,
}

impl<GridRef> Row<GridRef> {
    #[inline]
    pub(crate) fn new(grid: GridRef, y: usize) -> Self {
        Self { grid, y }
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.y
    }
}

impl<'a, G> Deref for Row<&'a mut G> {
    type Target = Row<&'a G>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a, G> From<Row<&'a mut G>> for Row<&'a G> {
    #[inline]
    fn from(Row { grid, y }: Row<&'a mut G>) -> Self {
        Self { grid, y }
    }
}

impl<'a, G> From<&'a Row<&'a G>> for Row<&'a G> {
    #[inline]
    fn from(Row { grid, y }: &'a Row<&'a G>) -> Self {
        Self { grid, y: *y }
    }
}

impl<'a, G> From<&'a Row<&'a mut G>> for Row<&'a G> {
    #[inline]
    fn from(Row { grid, y }: &'a Row<&'a mut G>) -> Self {
        Self { grid, y: *y }
    }
}

impl<'a, G: Grid> Row<&'a G> {
    #[inline]
    pub fn len(&self) -> usize {
        self.grid.width()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn get(&self, x: usize) -> Option<&G::Item> {
        self.grid.get(x, self.y)
    }

    /// Returns a reference to the item at position `x` in the row without bounds checking.
    ///
    /// For a safe alternative, see [`get`](Self::get).
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    pub unsafe fn get_unchecked(&self, x: usize) -> &G::Item {
        self.grid.get_unchecked(x, self.y)
    }

    #[inline]
    pub fn as_slice(&self) -> Option<&[G::Item]> {
        self.grid.row_slice(self.y)
    }

    #[inline]
    pub fn iter(&self) -> RowIter<&'a G> {
        RowIter::new(self.grid, self.y, self.len())
    }
}

impl<'a, G: GridMut> Row<&'a mut G> {
    #[inline]
    pub fn get_mut(&mut self, x: usize) -> Option<&mut G::Item> {
        self.grid.get_mut(x, self.y)
    }

    /// # Safety
    /// Because this row has a mutable reference to the grid itself, it can
    /// give out a mutable reference to a value inside it.
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, x: usize) -> &mut G::Item {
        self.grid.get_unchecked_mut(x, self.y)
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> Option<&mut [G::Item]> {
        self.grid.row_slice_mut(self.y)
    }

    #[inline]
    pub fn fill_with<F: FnMut() -> G::Item>(&mut self, mut f: F) {
        if let Some(slice) = self.as_mut_slice() {
            slice.fill_with(f);
        } else {
            for val in self.iter_mut() {
                *val = f();
            }
        }
    }

    #[inline]
    pub fn fill(&mut self, value: G::Item)
    where
        G::Item: Clone,
    {
        if self.is_empty() {
            return;
        }
        if let Some(slice) = self.as_mut_slice() {
            slice.fill(value);
        } else {
            for x in 1..self.len() {
                *self.get_mut(x).unwrap() = value.clone();
            }
            *self.get_mut(0).unwrap() = value;
        }
    }

    #[inline]
    pub fn clone_from<G2>(&mut self, row: impl Into<Row<&'a G2>>)
    where
        G2: Grid<Item = G::Item> + 'a,
        G::Item: Clone,
    {
        let row = row.into();
        assert_eq!(self.len(), row.len());
        match (self.as_mut_slice(), row.as_slice()) {
            (Some(dst), Some(src)) => {
                dst.clone_from_slice(src);
            }
            (Some(dst), None) => {
                for (dst, src) in dst.iter_mut().zip(row) {
                    *dst = src.clone();
                }
            }
            (None, Some(src)) => {
                for (dst, src) in self.iter_mut().zip(src) {
                    *dst = src.clone();
                }
            }
            (None, None) => {
                for (dst, src) in self.iter_mut().zip(row) {
                    *dst = src.clone();
                }
            }
        }
    }

    #[inline]
    pub fn copy_from<G2>(&mut self, row: impl Into<Row<&'a G2>>)
    where
        G2: Grid<Item = G::Item> + 'a,
        G::Item: Copy,
    {
        let row = row.into();
        assert_eq!(self.len(), row.len());
        match (self.as_mut_slice(), row.as_slice()) {
            (Some(dst), Some(src)) => {
                dst.copy_from_slice(src);
            }
            (Some(dst), None) => {
                for (dst, src) in dst.iter_mut().zip(row) {
                    *dst = *src;
                }
            }
            (None, Some(src)) => {
                for (dst, src) in self.iter_mut().zip(src) {
                    *dst = *src;
                }
            }
            (None, None) => {
                for (dst, src) in self.iter_mut().zip(row) {
                    *dst = *src;
                }
            }
        }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> RowIter<&mut G> {
        RowIter::new(self.grid, self.y, self.len())
    }
}

impl<'a, G: Grid> IntoIterator for Row<&'a G> {
    type Item = &'a G::Item;
    type IntoIter = RowIter<&'a G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RowIter::new(self.grid, self.y, self.grid.width())
    }
}

impl<'a, G: GridMut> IntoIterator for Row<&'a mut G> {
    type Item = &'a mut G::Item;
    type IntoIter = RowIter<&'a mut G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let w = self.grid.width();
        RowIter::new(self.grid, self.y, w)
    }
}

impl<'a, G: Grid> IntoIterator for &'a Row<&'a G> {
    type Item = &'a G::Item;
    type IntoIter = RowIter<&'a G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RowIter::new(self.grid, self.y, self.grid.width())
    }
}

impl<'a, G: Grid> IntoIterator for &'a Row<&'a mut G> {
    type Item = &'a G::Item;
    type IntoIter = RowIter<&'a G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RowIter::new(self.grid, self.y, self.grid.width())
    }
}

impl<'a, G: GridMut> IntoIterator for &'a mut Row<&'a mut G> {
    type Item = &'a mut G::Item;
    type IntoIter = RowIter<&'a mut G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let w = self.grid.width();
        RowIter::new(self.grid, self.y, w)
    }
}
