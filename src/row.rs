use crate::{Grid, GridMut, RowIter};
use std::ops::Deref;

#[derive(Clone)]
pub struct Row<GridRef> {
    pub(crate) grid: GridRef,
    pub(crate) y: usize,
}

impl<GridRef> Row<GridRef> {
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
    pub fn get(&self, x: usize) -> Option<&G::Item> {
        self.grid.get(x, self.y)
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, x: usize) -> &G::Item {
        self.grid.get_unchecked(x, self.y)
    }

    #[inline]
    pub fn as_slice(&self) -> Option<&[G::Item]> {
        self.grid.row_slice(self.y)
    }

    #[inline]
    pub fn iter(&self) -> RowIter<&Self> {
        let w = self.len();
        RowIter { row: self, x: 0, w }
    }
}

impl<'a, G: GridMut> Row<&'a mut G> {
    #[inline]
    pub fn get_mut(&mut self, x: usize) -> Option<&mut G::Item> {
        self.grid.get_mut(x, self.y)
    }

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
            for x in 0..self.len() {
                *self.get_mut(x).unwrap() = f();
            }
        }
    }

    #[inline]
    pub fn fill(&mut self, value: G::Item)
    where
        G::Item: Clone,
    {
        if self.len() == 0 {
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
                for x in 0..row.len() {
                    dst[x] = row.get(x).unwrap().clone();
                }
            }
            (None, Some(src)) => {
                for x in 0..row.len() {
                    *self.get_mut(x).unwrap() = src[x].clone();
                }
            }
            (None, None) => {
                for i in 0..self.len() {
                    *self.get_mut(i).unwrap() = row.get(i).unwrap().clone();
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
                for x in 0..row.len() {
                    dst[x] = *row.get(x).unwrap();
                }
            }
            (None, Some(src)) => {
                for x in 0..row.len() {
                    *self.get_mut(x).unwrap() = src[x];
                }
            }
            (None, None) => {
                for i in 0..self.len() {
                    *self.get_mut(i).unwrap() = *row.get(i).unwrap();
                }
            }
        }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> RowIter<&mut Self> {
        let w = self.len();
        RowIter { row: self, x: 0, w }
    }
}

impl<'a, G: Grid> IntoIterator for &'a Row<&'a G> {
    type Item = &'a G::Item;
    type IntoIter = RowIter<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RowIter {
            row: self,
            x: 0,
            w: self.len(),
        }
    }
}

impl<'a, G: Grid> IntoIterator for &'a Row<&'a mut G> {
    type Item = &'a G::Item;
    type IntoIter = RowIter<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let w = self.len();
        RowIter { row: self, x: 0, w }
    }
}

impl<'a, G: GridMut> IntoIterator for &'a mut Row<&'a mut G> {
    type Item = &'a mut G::Item;
    type IntoIter = RowIter<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let w = self.len();
        RowIter { row: self, x: 0, w }
    }
}
