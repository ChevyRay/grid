use crate::{GridView, GridViewMut};

pub trait Grid {
    type Item;
    type Root;

    fn root(&self) -> &Self::Root;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get(&self, x: usize, y: usize) -> Option<&Self::Item>;
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Item;
    fn row_slice(&self, y: usize) -> Option<&[Self::Item]>;

    #[inline]
    fn root_x(&self) -> usize {
        0
    }

    #[inline]
    fn root_y(&self) -> usize {
        0
    }

    #[inline]
    fn try_view(&self, x: usize, y: usize, w: usize, h: usize) -> Option<GridView<'_, Self::Root>> {
        if x + w <= self.width() && y + h <= self.height() {
            let x = self.root_x() + x;
            let y = self.root_y() + y;
            Some(GridView::new(self.root(), x, y, w, h))
        } else {
            None
        }
    }

    #[inline]
    fn view(&self, x: usize, y: usize, w: usize, h: usize) -> GridView<'_, Self::Root> {
        self.try_view(x, y, w, h)
            .expect("view does not overlap grid's bounds")
    }

    #[inline]
    fn full_view(&self) -> GridView<'_, Self::Root>
    where
        Self::Root: Grid<Item = Self::Item>,
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

pub trait GridMut: Grid {
    fn root_mut(&mut self) -> &mut Self::Root;
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item>;
    unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut Self::Item;
    fn row_slice_mut(&mut self, y: usize) -> Option<&mut [Self::Item]>;

    #[inline]
    fn set(&mut self, x: usize, y: usize, value: Self::Item) -> Option<Self::Item> {
        self.get_mut(x, y)
            .map(|curr| std::mem::replace(curr, value))
    }

    #[inline]
    unsafe fn set_unchecked(&mut self, x: usize, y: usize, value: Self::Item) -> Self::Item {
        std::mem::replace(self.get_unchecked_mut(x, y), value)
    }

    #[inline]
    fn try_view_mut(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) -> Option<GridViewMut<'_, Self::Root>>
    where
        Self::Root: GridMut<Item = Self::Item>,
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
    fn view_mut(&mut self, x: usize, y: usize, w: usize, h: usize) -> GridViewMut<'_, Self::Root>
    where
        Self::Root: GridMut<Item = Self::Item>,
    {
        self.try_view_mut(x, y, w, h)
            .expect("view does not overlap grid's bounds")
    }

    #[inline]
    fn full_view_mut(&mut self) -> GridViewMut<'_, Self::Root>
    where
        Self::Root: GridMut<Item = Self::Item>,
    {
        let w = self.width();
        let h = self.height();
        GridViewMut::new(self.root_mut(), 0, 0, w, h)
    }
}
