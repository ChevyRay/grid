use crate::cols_iter::ColsIter;
use crate::{Col, GridBuf, GridIter, GridMut, Row, RowsIter, View};

/// A type representing an immutable 2D array.
pub trait Grid {
    type Item;
    type Root: Grid<Item = Self::Item>;

    fn root(&self) -> &Self::Root;
    fn root_x(&self) -> usize;
    fn root_y(&self) -> usize;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get(&self, x: usize, y: usize) -> Option<&Self::Item>;

    /// # Safety
    /// Calling this method with an out-of-bounds coord is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Item;

    fn row_slice(&self, y: usize) -> Option<&[Self::Item]>;

    #[inline]
    fn same_size<G2: Grid>(&self, other: &G2) -> bool {
        self.width() == other.width() && self.height() == other.height()
    }

    #[inline]
    fn try_view(&self, x: usize, y: usize, w: usize, h: usize) -> Option<View<&Self::Root>> {
        if x + w <= self.width() && y + h <= self.height() {
            let x = self.root_x() + x;
            let y = self.root_y() + y;
            Some(View::new(self.root(), x, y, w, h))
        } else {
            None
        }
    }

    #[inline]
    fn view(&self, x: usize, y: usize, w: usize, h: usize) -> View<&Self::Root> {
        self.try_view(x, y, w, h)
            .expect("view does not overlap grid's bounds")
    }

    #[inline]
    fn full_view(&self) -> View<&Self::Root> {
        self.view(0, 0, self.width(), self.height())
    }

    #[inline]
    fn to_grid_buf<S>(&self, store: S) -> GridBuf<Self::Item, S>
    where
        S: AsRef<[Self::Item]> + AsMut<[Self::Item]>,
        Self::Item: Clone,
        Self: Sized,
    {
        let mut buf = GridBuf::with_store(self.width(), self.height(), store);
        buf.clone_from(self);
        buf
    }

    #[inline]
    fn to_arr_buf<const N: usize>(&self) -> GridBuf<Self::Item, [Self::Item; N]>
    where
        Self::Item: Default + Clone,
        Self: Sized,
    {
        let mut arr = std::array::from_fn(|_| Self::Item::default());
        for (dst, src) in arr.chunks_exact_mut(self.width()).zip(self.rows()) {
            if let Some(src) = src.as_slice() {
                dst.clone_from_slice(src);
            } else {
                for (dst, src) in dst.iter_mut().zip(&src) {
                    *dst = src.clone();
                }
            }
        }
        GridBuf::with_store(self.width(), self.height(), arr)
    }

    fn to_vec_grid(&self) -> GridBuf<Self::Item, Vec<Self::Item>>
    where
        Self::Item: Clone,
        Self: Sized,
    {
        let mut vec = Vec::with_capacity(self.width() * self.height());
        for row in self.rows() {
            if let Some(row) = row.as_slice() {
                vec.extend_from_slice(row);
            } else {
                vec.extend(row.iter().cloned())
            }
        }
        GridBuf::with_store(self.width(), self.height(), vec)
    }

    #[inline]
    fn iter(&self) -> GridIter<&Self>
    where
        Self: Sized,
    {
        GridIter::new(self)
    }

    #[inline]
    fn cols(&self) -> ColsIter<&Self>
    where
        Self: Sized,
    {
        ColsIter::new(self, self.width())
    }

    #[inline]
    fn try_col(&self, x: usize) -> Option<Col<&Self>> {
        (x < self.width()).then(|| Col::new(self, x))
    }

    #[inline]
    fn col(&self, x: usize) -> Col<&Self> {
        self.try_col(x).expect("column index out of bounds")
    }

    #[inline]
    fn rows(&self) -> RowsIter<&Self>
    where
        Self: Sized,
    {
        RowsIter::new(self, self.height())
    }

    #[inline]
    fn try_row(&self, y: usize) -> Option<Row<&Self>> {
        (y < self.height()).then(|| Row::new(self, y))
    }

    #[inline]
    fn row(&self, y: usize) -> Row<&Self> {
        self.try_row(y).expect("row index out of bounds")
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

impl<T, const W: usize, const H: usize> Grid for [[T; W]; H] {
    type Item = T;
    type Root = Self;

    #[inline]
    fn root(&self) -> &Self::Root {
        self
    }

    #[inline]
    fn root_x(&self) -> usize {
        0
    }

    #[inline]
    fn root_y(&self) -> usize {
        0
    }

    #[inline]
    fn width(&self) -> usize {
        W
    }

    #[inline]
    fn height(&self) -> usize {
        H
    }

    #[inline]
    fn get(&self, x: usize, y: usize) -> Option<&Self::Item> {
        (x < W && y < H).then(|| &self[y][x])
    }

    #[inline]
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Item {
        self.as_slice().get_unchecked(y).get_unchecked(x)
    }

    #[inline]
    fn row_slice(&self, y: usize) -> Option<&[Self::Item]> {
        (y < H).then(|| self[y].as_slice())
    }
}
