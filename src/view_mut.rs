use crate::{Grid, GridMut, Iter, IterMut};

pub struct ViewMut<'a, G> {
    grid: &'a mut G,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl<'a, G: GridMut> ViewMut<'a, G> {
    pub(crate) fn new(grid: &'a mut G, x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { grid, x, y, w, h }
    }
}

impl<G: Grid> Grid for ViewMut<'_, G> {
    type Item = G::Item;
    type Root = G;

    #[inline]
    fn root(&self) -> &Self::Root {
        self.grid
    }

    #[inline]
    fn width(&self) -> usize {
        self.w
    }

    #[inline]
    fn height(&self) -> usize {
        self.h
    }

    #[inline]
    fn get(&self, x: usize, y: usize) -> Option<&Self::Item> {
        if x < self.w && y < self.h {
            self.grid.get(self.x + x, self.y + y)
        } else {
            None
        }
    }

    #[inline]
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Item {
        self.grid.get_unchecked(self.x + x, self.y + y)
    }

    #[inline]
    fn row_slice(&self, y: usize) -> Option<&[Self::Item]> {
        if y < self.h {
            self.grid
                .row_slice(self.y + y)
                .and_then(|s| s.get(self.x..(self.x + self.w)))
        } else {
            None
        }
    }

    #[inline]
    fn root_x(&self) -> usize {
        self.x
    }

    #[inline]
    fn root_y(&self) -> usize {
        self.y
    }
}

impl<G: GridMut> GridMut for ViewMut<'_, G> {
    #[inline]
    fn root_mut(&mut self) -> &mut Self::Root {
        self.grid
    }

    #[inline]
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item> {
        if x < self.w && y < self.h {
            self.grid.get_mut(self.x + x, self.y + y)
        } else {
            None
        }
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut Self::Item {
        self.grid.get_unchecked_mut(self.x + x, self.y + y)
    }

    #[inline]
    fn row_slice_mut(&mut self, y: usize) -> Option<&mut [Self::Item]> {
        if y < self.h {
            self.grid
                .row_slice_mut(self.y + y)
                .and_then(|s| s.get_mut(self.x..(self.x + self.w)))
        } else {
            None
        }
    }
}

impl<'a, G: Grid> IntoIterator for &'a ViewMut<'a, G> {
    type Item = (&'a G::Item, usize, usize);
    type IntoIter = Iter<'a, ViewMut<'a, G>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, G: GridMut> IntoIterator for &'a mut ViewMut<'a, G> {
    type Item = (&'a mut G::Item, usize, usize);
    type IntoIter = IterMut<'a, ViewMut<'a, G>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
