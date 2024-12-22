use crate::{ColIter, Grid, GridMut, Row};
use std::ops::Deref;

#[derive(Clone)]
pub struct Col<GridRef> {
    grid: GridRef,
    x: usize,
}

impl<GridRef> Col<GridRef> {
    #[inline]
    pub(crate) fn new(grid: GridRef, x: usize) -> Self {
        Self { grid, x }
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.x
    }
}

impl<'a, G> Deref for Col<&'a mut G> {
    type Target = Col<&'a G>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a, G> From<Col<&'a mut G>> for Col<&'a G> {
    #[inline]
    fn from(Col { grid, x }: Col<&'a mut G>) -> Self {
        Self { grid, x }
    }
}

impl<'a, G> From<&'a Col<&'a G>> for Col<&'a G> {
    #[inline]
    fn from(Col { grid, x }: &'a Col<&'a G>) -> Self {
        Self { grid, x: *x }
    }
}

impl<'a, G> From<&'a Col<&'a mut G>> for Col<&'a G> {
    #[inline]
    fn from(Col { grid, x }: &'a Col<&'a mut G>) -> Self {
        Self { grid, x: *x }
    }
}

impl<'a, G: Grid> Col<&'a G> {
    #[inline]
    pub fn len(&self) -> usize {
        self.grid.height()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn get(&self, y: usize) -> Option<&G::Item> {
        self.grid.get(self.x, y)
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
    pub unsafe fn get_unchecked(&self, y: usize) -> &G::Item {
        self.grid.get_unchecked(self.x, y)
    }

    #[inline]
    pub fn iter(&self) -> ColIter<&'a G> {
        ColIter::new(self.grid, self.x, self.len())
    }
}

impl<'a, G: GridMut> Col<&'a mut G> {
    #[inline]
    pub fn get_mut(&mut self, y: usize) -> Option<&mut G::Item> {
        self.grid.get_mut(self.x, y)
    }

    /// # Safety
    /// Because this row has a mutable reference to the grid itself, it can
    /// give out a mutable reference to a value inside it.
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, y: usize) -> &mut G::Item {
        self.grid.get_unchecked_mut(self.x, y)
    }

    #[inline]
    pub fn fill_with<F: FnMut() -> G::Item>(&mut self, mut f: F) {
        for val in self.iter_mut() {
            *val = f();
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
        for y in 1..self.len() {
            *self.get_mut(y).unwrap() = value.clone();
        }
        *self.get_mut(0).unwrap() = value;
    }

    #[inline]
    pub fn clone_from<G2>(&mut self, col: impl Into<Col<&'a G2>>)
    where
        G2: Grid<Item = G::Item> + 'a,
        G::Item: Clone,
    {
        let col = col.into();
        assert_eq!(self.len(), col.len());
        for (dst, src) in self.iter_mut().zip(col) {
            *dst = src.clone();
        }
    }

    #[inline]
    pub fn copy_from<G2>(&mut self, col: impl Into<Col<&'a G2>>)
    where
        G2: Grid<Item = G::Item> + 'a,
        G::Item: Copy,
    {
        let col = col.into();
        assert_eq!(self.len(), col.len());
        for (dst, src) in self.iter_mut().zip(col) {
            *dst = src.clone();
        }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> ColIter<&mut G> {
        ColIter::new(self.grid, self.x, self.len())
    }
}

impl<'a, G: Grid> IntoIterator for Col<&'a G> {
    type Item = &'a G::Item;
    type IntoIter = ColIter<&'a G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ColIter::new(self.grid, self.x, self.grid.height())
    }
}

impl<'a, G: GridMut> IntoIterator for Col<&'a mut G> {
    type Item = &'a mut G::Item;
    type IntoIter = ColIter<&'a mut G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let h = self.grid.height();
        ColIter::new(self.grid, self.x, h)
    }
}

impl<'a, G: Grid> IntoIterator for &'a Col<&'a G> {
    type Item = &'a G::Item;
    type IntoIter = ColIter<&'a G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ColIter::new(self.grid, self.x, self.grid.height())
    }
}

impl<'a, G: Grid> IntoIterator for &'a Col<&'a mut G> {
    type Item = &'a G::Item;
    type IntoIter = ColIter<&'a G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ColIter::new(self.grid, self.x, self.grid.height())
    }
}

impl<'a, G: GridMut> IntoIterator for &'a mut Col<&'a mut G> {
    type Item = &'a mut G::Item;
    type IntoIter = ColIter<&'a mut G>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let h = self.grid.height();
        ColIter::new(self.grid, self.x, h)
    }
}
