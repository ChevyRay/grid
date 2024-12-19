use crate::{Grid, GridMut};
use std::marker::PhantomData;

pub struct GridView<'a, T, G: Grid<T>> {
    grid: &'a G,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    marker: PhantomData<T>,
}

impl<'a, T, G: Grid<T>> GridView<'a, T, G> {
    #[inline]
    pub(crate) fn new(grid: &'a G, x: usize, y: usize, w: usize, h: usize) -> Self {
        Self {
            grid,
            x,
            y,
            w,
            h,
            marker: PhantomData,
        }
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

impl<'a, T, G: Grid<T>> Grid<T> for GridView<'a, T, G> {
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
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.w && y < self.h {
            self.root().get(self.x + x, self.y + y)
        } else {
            None
        }
    }

    #[inline]
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &T {
        self.root().get_unchecked(self.x + x, self.y + y)
    }

    #[inline]
    fn row_slice(&self, y: usize) -> Option<&[T]> {
        if y < self.h {
            self.grid
                .row_slice(self.y + y)
                .and_then(|s| s.get(self.x..(self.x + self.w)))
        } else {
            None
        }
    }

    fn try_view(
        &self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) -> Option<GridView<'_, T, Self::Root>>
    where
        Self::Root: Grid<T>,
    {
        if x + w <= self.w && y + h <= self.h {
            Some(GridView::new(self.grid, self.x + x, self.y + y, w, h))
        } else {
            None
        }
    }
}

pub struct GridViewMut<'a, T, G: Grid<T>> {
    grid: &'a mut G,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    marker: PhantomData<T>,
}

impl<'a, T, G: GridMut<T>> GridViewMut<'a, T, G> {
    pub(crate) fn new(grid: &'a mut G, x: usize, y: usize, w: usize, h: usize) -> Self {
        Self {
            grid,
            x,
            y,
            w,
            h,
            marker: PhantomData,
        }
    }
}

impl<'a, T, G: Grid<T>> Grid<T> for GridViewMut<'a, T, G> {
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
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.w && y < self.h {
            self.grid.get(self.x + x, self.y + y)
        } else {
            None
        }
    }

    #[inline]
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &T {
        self.grid.get_unchecked(self.x + x, self.y + y)
    }

    #[inline]
    fn row_slice(&self, y: usize) -> Option<&[T]> {
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

impl<'a, T, G: GridMut<T>> GridMut<T> for GridViewMut<'a, T, G> {
    #[inline]
    fn root_mut(&mut self) -> &mut Self::Root {
        self.grid
    }

    #[inline]
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.w && y < self.h {
            self.grid.get_mut(self.x + x, self.y + y)
        } else {
            None
        }
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.grid.get_unchecked_mut(self.x + x, self.y + y)
    }

    #[inline]
    fn row_slice_mut(&mut self, y: usize) -> Option<&mut [T]> {
        if y < self.h {
            self.grid
                .row_slice_mut(self.y + y)
                .and_then(|s| s.get_mut(self.x..(self.x + self.w)))
        } else {
            None
        }
    }
}
