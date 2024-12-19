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
        if x + w <= self.width() && y + h <= self.height() {
            let x = self.root_x() + x;
            let y = self.root_y() + y;
            Some(GridView::new(self.root(), x, y, w, h))
        } else {
            None
        }
    }

    #[inline]
    fn view(&self, x: usize, y: usize, w: usize, h: usize) -> GridView<'_, T, Self::Root>
    where
        Self::Root: Grid<T>,
    {
        self.try_view(x, y, w, h)
            .expect("view does not overlap grid's bounds")
    }

    #[inline]
    fn full_view(&self) -> GridView<'_, T, Self::Root>
    where
        Self::Root: Grid<T>,
    {
        GridView::new(self.root(), 0, 0, self.width(), self.height())
    }

    // IDEA: getting views from ranges could also work
    /*fn view(
        &self,
        cols: impl RangeBounds<usize>,
        rows: impl RangeBounds<usize>,
    ) -> Option<GridView<'_, T, Self::Root>>
    where
        Self::Root: Grid<T>,
    {
        fn bound_to_range(bound: impl RangeBounds<usize>, max: usize) -> Option<(usize, usize)> {
            let lo = match bound.start_bound() {
                Bound::Included(&lo) => lo,
                Bound::Excluded(&lo) => lo.checked_add(1)?,
                Bound::Unbounded => 0,
            };
            let hi = match bound.end_bound() {
                Bound::Included(&hi) => hi.checked_add(1)?,
                Bound::Excluded(&hi) => hi,
                Bound::Unbounded => max,
            };
            let len = hi.checked_sub(lo)?;
            (lo + len <= max).then(|| (lo, len))
        }
        let (x, w) = bound_to_range(cols, self.width())?;
        let (y, h) = bound_to_range(rows, self.height())?;
        let x = self.root_x() + x;
        let y = self.root_y() + y;
        Some(GridView::new(self.root(), x, y, w, h))
    }*/
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
    fn try_view_mut(
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

    #[inline]
    fn view_mut(&mut self, x: usize, y: usize, w: usize, h: usize) -> GridViewMut<'_, T, Self::Root>
    where
        Self::Root: GridMut<T>,
    {
        self.try_view_mut(x, y, w, h)
            .expect("view does not overlap grid's bounds")
    }

    #[inline]
    fn full_view_mut(&mut self) -> GridViewMut<'_, T, Self::Root>
    where
        Self::Root: GridMut<T>,
    {
        let w = self.width();
        let h = self.height();
        GridViewMut::new(self.root_mut(), 0, 0, w, h)
    }
}
