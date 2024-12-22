use crate::{Grid, GridMut};
use std::ops::Deref;

#[derive(Clone)]
pub struct Rows<GridRef> {
    pub(crate) grid: GridRef,
    pub(crate) y: usize,
}

impl<'a, G: Grid> Iterator for Rows<&'a G> {
    type Item = Row<&'a G>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.grid.height() {
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
}

impl<'a, G: GridMut> Iterator for Rows<&'a mut G> {
    type Item = Row<&'a mut G>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.grid.height() {
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
}

#[derive(Clone)]
pub struct Row<GridRef> {
    grid: GridRef,
    y: usize,
}

impl<GridRef> Row<GridRef> {
    #[inline]
    pub fn index(&self) -> usize {
        self.y
    }
}

impl<'a, G> Deref for Row<&'a mut G> {
    type Target = Row<&'a G>;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a, G> From<Row<&'a mut G>> for Row<&'a G> {
    fn from(Row { grid, y }: Row<&'a mut G>) -> Self {
        Self { grid, y }
    }
}

impl<'a, G> From<&'a Row<&'a G>> for Row<&'a G> {
    fn from(Row { grid, y }: &'a Row<&'a G>) -> Self {
        Self { grid, y: *y }
    }
}

impl<'a, G> From<&'a Row<&'a mut G>> for Row<&'a G> {
    fn from(Row { grid, y }: &'a Row<&'a mut G>) -> Self {
        Self { grid, y: *y }
    }
}

impl<'a, G: Grid> Row<&'a G> {
    pub fn len(&self) -> usize {
        self.grid.width()
    }

    pub fn get(&self, x: usize) -> Option<&G::Item> {
        self.grid.get(x, self.y)
    }

    pub unsafe fn get_unchecked(&self, x: usize) -> &G::Item {
        self.grid.get_unchecked(x, self.y)
    }

    pub fn as_slice(&self) -> Option<&[G::Item]> {
        self.grid.row_slice(self.y)
    }

    #[inline]
    pub fn iter(&self) -> RowIter<&Self> {
        RowIter { row: self, x: 0 }
    }
}

impl<'a, G: GridMut> Row<&'a mut G> {
    pub fn get_mut(&mut self, x: usize) -> Option<&mut G::Item> {
        self.grid.get_mut(x, self.y)
    }

    pub unsafe fn get_unchecked_mut(&mut self, x: usize) -> &mut G::Item {
        self.grid.get_unchecked_mut(x, self.y)
    }

    pub fn as_mut_slice(&mut self) -> Option<&mut [G::Item]> {
        self.grid.row_slice_mut(self.y)
    }

    pub fn fill_with<F: FnMut() -> G::Item>(&mut self, mut f: F) {
        if let Some(slice) = self.as_mut_slice() {
            slice.fill_with(f);
        } else {
            for x in 0..self.len() {
                *self.get_mut(x).unwrap() = f();
            }
        }
    }

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
        RowIter { row: self, x: 0 }
    }
}

impl<'a, G: Grid> IntoIterator for &'a Row<&'a G> {
    type Item = &'a G::Item;
    type IntoIter = RowIter<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RowIter { row: self, x: 0 }
    }
}

impl<'a, G: Grid> IntoIterator for &'a Row<&'a mut G> {
    type Item = &'a G::Item;
    type IntoIter = RowIter<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RowIter { row: self, x: 0 }
    }
}

impl<'a, G: GridMut> IntoIterator for &'a mut Row<&'a mut G> {
    type Item = &'a mut G::Item;
    type IntoIter = RowIter<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RowIter { row: self, x: 0 }
    }
}

#[derive(Clone)]
pub struct RowIter<RowRef> {
    row: RowRef,
    x: usize,
}

impl<'a, G: Grid> Iterator for RowIter<&'a Row<&'a G>> {
    type Item = &'a G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.row.get(self.x)?;
        self.x += 1;
        Some(val)
    }
}

impl<'a, G: Grid> Iterator for RowIter<&'a Row<&'a mut G>> {
    type Item = &'a G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.row.get(self.x)?;
        self.x += 1;
        Some(val)
    }
}

impl<'a, G: GridMut> Iterator for RowIter<&'a mut Row<&'a mut G>> {
    type Item = &'a mut G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let val: *mut G::Item = self.row.get_mut(self.x)?;
        self.x += 1;
        Some(unsafe { &mut *val })
    }
}
