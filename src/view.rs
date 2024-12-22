use crate::{Grid, GridBuf, GridMut};

pub struct View<GridRef> {
    pub(crate) grid: GridRef,
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) w: usize,
    pub(crate) h: usize,
}

impl<'a, G> View<&'a G> {
    #[inline]
    pub const fn x(&self) -> usize {
        self.x
    }

    #[inline]
    pub const fn y(&self) -> usize {
        self.y
    }

    pub fn to_grid_buf<S>(&self, store: S) -> GridBuf<G::Item, S>
    where
        G: Grid<Item: Clone>,
        S: AsRef<[G::Item]> + AsMut<[G::Item]>,
        Self: Sized,
    {
        let mut buf = GridBuf::with_store(self.w, self.h, store);
        buf.clone_from(self);
        buf
    }
}

impl<'a, G: Grid> Grid for View<&'a G> {
    type Item = G::Item;
    type Root = G;

    #[inline]
    fn root(&self) -> &Self::Root {
        self.grid
    }

    #[inline]
    fn root_x(&self) -> usize {
        self.x
    }

    #[inline]
    fn root_y(&self) -> usize {
        self.y
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
    fn try_view(&self, x: usize, y: usize, w: usize, h: usize) -> Option<View<&Self::Root>> {
        if x + w <= self.w && y + h <= self.h {
            Some(View {
                grid: self.grid,
                x: self.x + x,
                y: self.y + y,
                w,
                h,
            })
        } else {
            None
        }
    }
}

impl<'a, G: Grid> Grid for View<&'a mut G> {
    type Item = G::Item;
    type Root = G;

    #[inline]
    fn root(&self) -> &Self::Root {
        self.grid
    }

    #[inline]
    fn root_x(&self) -> usize {
        self.x
    }

    #[inline]
    fn root_y(&self) -> usize {
        self.y
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
    fn try_view(&self, x: usize, y: usize, w: usize, h: usize) -> Option<View<&Self::Root>> {
        if x + w <= self.w && y + h <= self.h {
            Some(View {
                grid: self.grid,
                x: self.x + x,
                y: self.y + y,
                w,
                h,
            })
        } else {
            None
        }
    }
}

impl<'a, G: GridMut> GridMut for View<&'a mut G> {
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

    #[inline]
    fn try_view_mut(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) -> Option<View<&mut Self::Root>> {
        if x + w <= self.w && y + h <= self.h {
            Some(View {
                grid: self.grid,
                x: self.x + x,
                y: self.y + y,
                w,
                h,
            })
        } else {
            None
        }
    }
}
