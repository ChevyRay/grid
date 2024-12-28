use crate::{Grid, GridMut};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

/// Sub-section of a larger grid.
#[repr(C)]
#[derive(Clone)]
pub struct View<GridRef> {
    grid: GridRef,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl<GridRef> View<GridRef> {
    pub(crate) fn new(grid: GridRef, x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { grid, x, y, w, h }
    }
}

impl<'a, G> Deref for View<&'a mut G> {
    type Target = View<&'a G>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a, G> From<View<&'a mut G>> for View<&'a G> {
    #[inline]
    fn from(View { grid, x, y, w, h }: View<&'a mut G>) -> Self {
        Self { grid, x, y, w, h }
    }
}

impl<'a, G> From<&'a View<&'a G>> for View<&'a G> {
    #[inline]
    fn from(View { grid, x, y, w, h }: &'a View<&'a G>) -> Self {
        Self::new(grid, *x, *y, *w, *h)
    }
}

impl<'a, G> From<&'a View<&'a mut G>> for View<&'a G> {
    #[inline]
    fn from(View { grid, x, y, w, h }: &'a View<&'a mut G>) -> Self {
        Self::new(grid, *x, *y, *w, *h)
    }
}

impl<G: Grid> Grid for View<&G> {
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
}

impl<G: Grid> Grid for View<&mut G> {
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
}

impl<G: GridMut> GridMut for View<&mut G> {
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

impl<A: Grid, B: Grid> PartialEq<View<&B>> for View<&A>
where
    A::Item: PartialEq<B::Item>,
{
    #[inline]
    fn eq(&self, other: &View<&B>) -> bool {
        self.eq_grid(other)
    }
}

impl<G: Grid> Debug for View<&G>
where
    G::Item: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.debug_fmt(f)
    }
}
