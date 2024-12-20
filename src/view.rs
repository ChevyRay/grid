use crate::{Grid, GridMut};

pub struct GridView<'a, G> {
    grid: &'a G,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl<'a, G> GridView<'a, G> {
    #[inline]
    pub(crate) fn new(grid: &'a G, x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { grid, x, y, w, h }
    }

    #[inline]
    pub fn x(&self) -> usize {
        self.x
    }

    #[inline]
    pub const fn y(&self) -> usize {
        self.y
    }
}

impl<G: Grid> Grid for GridView<'_, G> {
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
            self.root().get(self.x + x, self.y + y)
        } else {
            None
        }
    }

    #[inline]
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Item {
        self.root().get_unchecked(self.x + x, self.y + y)
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
    fn try_view(&self, x: usize, y: usize, w: usize, h: usize) -> Option<GridView<'_, Self::Root>> {
        if x + w <= self.w && y + h <= self.h {
            Some(GridView::new(self.grid, self.x + x, self.y + y, w, h))
        } else {
            None
        }
    }
}

pub struct GridViewMut<'a, G> {
    grid: &'a mut G,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl<'a, G: GridMut> GridViewMut<'a, G> {
    pub(crate) fn new(grid: &'a mut G, x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { grid, x, y, w, h }
    }
}

impl<G: Grid> Grid for GridViewMut<'_, G> {
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

impl<G: GridMut> GridMut for GridViewMut<'_, G> {
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
