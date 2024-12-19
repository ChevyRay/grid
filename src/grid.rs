use crate::{GridView, GridViewMut};

pub trait Grid<T> {
    type Root;

    fn root(&self) -> &Self::Root;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get(&self, x: usize, y: usize) -> Option<&T>;
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &T;
    fn row_slice(&self, y: usize) -> Option<&[T]>;

    #[inline]
    fn root_x(&self) -> usize {
        0
    }

    #[inline]
    fn root_y(&self) -> usize {
        0
    }

    #[inline]
    fn view(&self, x: usize, y: usize, w: usize, h: usize) -> Option<GridView<'_, T, Self::Root>>
    where
        Self::Root: Grid<T>,
    {
        if x + w <= self.width() && y + h <= self.height() {
            let x = self.root_x() + x;
            let y = self.root_y() + y;
            Some(GridView::new(self.root(), x, y, w, h))
        } else {
            None
        }
    }
}

pub trait GridMut<T>: Grid<T> {
    fn root_mut(&mut self) -> &mut Self::Root;
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T>;
    unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut T;
    fn row_slice_mut(&mut self, y: usize) -> Option<&mut [T]>;

    #[inline]
    fn set(&mut self, x: usize, y: usize, value: T) -> Option<T> {
        self.get_mut(x, y)
            .map(|curr| std::mem::replace(curr, value))
    }

    #[inline]
    unsafe fn set_unchecked(&mut self, x: usize, y: usize, value: T) -> T {
        std::mem::replace(self.get_unchecked_mut(x, y), value)
    }

    #[inline]
    fn view_mut(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) -> Option<GridViewMut<'_, T, Self::Root>>
    where
        Self::Root: GridMut<T>,
    {
        if x + w <= self.width() && y + h <= self.height() {
            let x = self.root_x() + x;
            let y = self.root_y() + y;
            Some(GridViewMut::new(self.root_mut(), x, y, w, h))
        } else {
            None
        }
    }
}
