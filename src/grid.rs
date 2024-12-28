use crate::cols_iter::ColsIter;
use crate::{ArrGrid, Col, GridBuf, GridIter, GridMut, Row, RowsIter, VecGrid, View};
use std::fmt::{Debug, Write};

/// A type representing an immutable 2D array.
pub trait Grid {
    /// The type of item this grid contains.
    type Item;

    /// The root grid type. [Views](View) use this to store a reference to the root
    /// grid so they can read and modify it.
    type Root: Grid<Item = Self::Item>;

    /// The root grid for this one. If this grid is the root, this returns `self`.
    ///
    /// ```
    /// # use grid::Grid;
    /// use std::ptr;
    ///
    /// let nums = [
    ///     [1, 2],
    ///     [3, 4],
    /// ];
    ///
    /// let nums_view = nums.view(0, 0, 2, 2);
    ///
    /// assert!(ptr::addr_eq(&nums, nums.root()));
    /// assert!(ptr::addr_eq(&nums, nums_view.root()));
    /// assert!(!ptr::addr_eq(&nums_view, nums_view.root()));
    /// ```
    fn root(&self) -> &Self::Root;

    /// This grid's x-offset from the root grid. For the root grid this is `0`.
    ///
    /// ```
    /// # use grid::Grid;
    /// let nums = [
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8],
    /// ];
    ///
    /// let nums_view = nums.view(1, 2, 2, 1);
    ///
    /// assert_eq!(nums.root_x(), 0);
    /// assert_eq!(nums_view.root_x(), 1);
    /// ```
    fn root_x(&self) -> usize;

    /// This grid's y-offset from the root grid. For the root grid this is `0`.
    ///
    ///
    /// ```
    /// # use grid::Grid;
    /// let nums = [
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8],
    /// ];
    ///
    /// let nums_view = nums.view(1, 2, 2, 1);
    ///
    /// assert_eq!(nums.root_y(), 0);
    /// assert_eq!(nums_view.root_y(), 2);
    /// ```
    fn root_y(&self) -> usize;

    /// Width of the grid (how many columns it has).
    ///
    /// ```
    /// # use grid::Grid;
    /// let nums = [
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8],
    /// ];
    ///
    /// let nums_view = nums.view(1, 2, 2, 1);
    ///
    /// assert_eq!(nums.width(), 3);
    /// assert_eq!(nums_view.width(), 2);
    /// ```
    fn width(&self) -> usize;

    /// Height of the grid (how many rows it has).
    ///
    /// ```
    /// # use grid::Grid;
    /// let nums = [
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8],
    /// ];
    ///
    /// let nums_view = nums.view(1, 2, 2, 1);
    ///
    /// assert_eq!(nums.height(), 3);
    /// assert_eq!(nums_view.height(), 1);
    /// ```
    fn height(&self) -> usize;

    /// Returns a reference to the value stored at `(x, y)` in the grid, or `None` if
    /// the provided coordinate is out of bounds.
    ///
    /// ```
    /// # use grid::Grid;
    /// let nums = [
    ///     ['A', 'B'],
    ///     ['C', 'D'],
    /// ];
    ///
    /// assert_eq!(nums.get(0, 0), Some(&'A'));
    /// assert_eq!(nums.get(1, 0), Some(&'B'));
    /// assert_eq!(nums.get(0, 1), Some(&'C'));
    /// assert_eq!(nums.get(1, 1), Some(&'D'));
    /// assert_eq!(nums.get(5, 0), None);
    /// assert_eq!(nums.get(2, 99), None);
    /// ```
    fn get(&self, x: usize, y: usize) -> Option<&Self::Item>;

    /// Returns a reference to the value stored at `(x, y)` in the grid, skipping
    /// any bounds checks.
    ///
    /// For a safe alternative, see [`get`](Self::get).
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds coord is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    ///
    /// # Example
    ///
    /// ```
    /// # use grid::Grid;
    /// let nums = [
    ///     ['A', 'B'],
    ///     ['C', 'D'],
    /// ];
    ///
    /// unsafe {
    ///     assert_eq!(nums.get_unchecked(0, 0), &'A');
    ///     assert_eq!(nums.get_unchecked(1, 0), &'B');
    ///     assert_eq!(nums.get_unchecked(0, 1), &'C');
    ///     assert_eq!(nums.get_unchecked(1, 1), &'D');
    /// }
    /// ```
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Item;

    /// Returns row `y` of the grid as a slice if it is able to do so. Algorithms that work
    /// on large portions of the grid may use this to look for performance gain. For example,
    /// [`Row::draw_copied`] uses this internally to call [`copy_from_slice`] when possible,
    /// which can be faster than manually copying elements one-by-one.
    ///
    /// [`copy_from_slice`]: https://doc.rust-lang.org/std/primitive.slice.html#method.copy_from_slice
    ///
    /// # Example
    ///
    /// ```
    /// # use grid::Grid;
    /// let nums = [
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8],
    /// ];
    ///
    /// assert_eq!(nums.row_slice(0), Some([0, 1, 2].as_slice()));
    /// assert_eq!(nums.row_slice(1), Some([3, 4, 5].as_slice()));
    /// assert_eq!(nums.row_slice(2), Some([6, 7, 8].as_slice()));
    /// assert_eq!(nums.row_slice(3), None);
    /// ```
    fn row_slice(&self, y: usize) -> Option<&[Self::Item]>;

    /// Returns true if both grids are the same size.
    ///
    /// ```
    /// # use grid::Grid;
    /// let nums = [
    ///     [1, 2],
    ///     [3, 4],
    /// ];
    ///
    /// assert!(nums.same_size(&[
    ///     [0, 0],
    ///     [0, 0],
    /// ]));
    /// assert!(!nums.same_size(&[
    ///     [0, 0, 0],
    ///     [0, 0, 0],
    /// ]));
    /// assert!(!nums.same_size(&[
    ///     [0, 0],
    ///     [0, 0],
    ///     [0, 0],
    /// ]));
    /// ```
    #[inline]
    fn same_size<G2: Grid>(&self, other: &G2) -> bool {
        self.width() == other.width() && self.height() == other.height()
    }

    /// Get an immutable [`View`] into this grid, or `None` if the provided region is
    /// out of bounds.
    ///
    /// ```
    /// # use grid::Grid;
    /// let nums = [
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8],
    /// ];
    ///
    /// assert_eq!(nums.get(0, 0), Some(&0));
    /// assert_eq!(nums.get(1, 1), Some(&4));
    ///
    /// let view = nums.try_view(1, 1, 2, 2).unwrap();
    /// assert_eq!(view.get(0, 0), Some(&4));
    /// assert_eq!(view.get(1, 1), Some(&8));
    ///
    /// assert!(nums.try_view(2, 2, 5, 5).is_none());
    /// ```
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

    /// Get an immutable [`View`] into this grid. Panicks if the provided region is out
    /// of bounds.
    #[inline]
    fn view(&self, x: usize, y: usize, w: usize, h: usize) -> View<&Self::Root> {
        self.try_view(x, y, w, h)
            .expect("view does not overlap grid's bounds")
    }

    /// Create a [`GridBuf`] using the provided storage and clone this entire
    /// grid into it. The resulting grid will be the same size as this one.
    #[inline]
    fn to_grid_buf<S>(&self, store: S) -> GridBuf<Self::Item, S>
    where
        S: AsRef<[Self::Item]> + AsMut<[Self::Item]>,
        Self::Item: Clone,
        Self: Sized,
    {
        let mut buf = GridBuf::with_store(self.width(), self.height(), store);
        buf.draw_cloned(self);
        buf
    }

    /// Create a stack-allocated [`GridBuf`], using an `N`-sized array for storage,
    /// and clone this entire grid into it. Panics if `N` is not exactly the area
    /// of the grid (`width * height`).
    #[inline]
    fn to_arr_grid<const N: usize>(&self) -> ArrGrid<Self::Item, N>
    where
        Self::Item: Default + Clone,
        Self: Sized,
    {
        assert_eq!(self.width() * self.height(), N);
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

    /// Create a stack-allocated [`GridBuf`], using a [`Vec`] for storage, and
    /// clone this entire grid into it.
    fn to_vec_grid(&self) -> VecGrid<Self::Item>
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

    /// Iterate over all values in the grid, with their positions.
    #[inline]
    fn iter(&self) -> GridIter<&Self>
    where
        Self: Sized,
    {
        GridIter::new(self)
    }

    /// Iterate over all columns in the grid.
    #[inline]
    fn cols(&self) -> ColsIter<&Self>
    where
        Self: Sized,
    {
        ColsIter::new(self, self.width())
    }

    /// Return the column `x`, or `None` if `x` is out of bounds.
    #[inline]
    fn try_col(&self, x: usize) -> Option<Col<&Self>> {
        (x < self.width()).then(|| Col::new(self, x))
    }

    /// Return the column `x`. Panics if `x` is out of bounds.
    #[inline]
    fn col(&self, x: usize) -> Col<&Self> {
        self.try_col(x).expect("column index out of bounds")
    }

    /// Iterate over the rows of the grid.
    #[inline]
    fn rows(&self) -> RowsIter<&Self>
    where
        Self: Sized,
    {
        RowsIter::new(self, self.height())
    }

    /// Return the row `y`, or `None` if `y` is out of bounds.
    #[inline]
    fn try_row(&self, y: usize) -> Option<Row<&Self>> {
        (y < self.height()).then(|| Row::new(self, y))
    }

    /// Return the row `y`. Panics if `y` is out of bounds.
    #[inline]
    fn row(&self, y: usize) -> Row<&Self> {
        self.try_row(y).expect("row index out of bounds")
    }

    #[inline]
    fn eq_grid<'a, H: Grid>(&'a self, other: &'a H) -> bool
    where
        RowsIter<&'a Self>: PartialEq<RowsIter<&'a H>>,
        Self: Sized,
    {
        self.rows() == other.rows()
    }

    #[inline]
    fn debug_fmt<W: Write>(&self, mut f: W) -> std::fmt::Result
    where
        Self::Item: Debug,
    {
        let mut s = String::new();
        let mut len = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                let val = self.get(x, y).unwrap();
                s.clear();
                write!(s, "{:?}", val)?;
                len = len.max(s.len());
            }
        }
        for y in 0..self.height() {
            for x in 0..self.width() {
                let val = self.get(x, y).unwrap();
                s.clear();
                write!(s, "{:?}", val)?;
                while s.len() < len {
                    s.push(' ');
                }
                write!(f, "[{}]", s)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
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
