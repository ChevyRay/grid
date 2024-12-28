use crate::cols_iter::ColsIter;
use crate::{Col, Grid, GridIter, Row, RowsIter, View};

/// A type representing a mutable 2D array.
pub trait GridMut: Grid {
    /// The root grid for this one. If this grid is the root, this returns `self`.
    fn root_mut(&mut self) -> &mut Self::Root;

    /// Returns a mutable reference to the value stored at `(x, y)` in the grid,
    /// or `None` if the provided coordinate is out of bounds.
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item>;

    /// Returns a mujtable reference to the value stored at `(x, y)` in the grid,
    /// skipping any bounds checks.
    ///
    /// For a safe alternative, see [`get_mut`](Self::get_mut).
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds coord is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut Self::Item;

    /// Returns row `y` of the grid as a mutable slice if it is able to do so. Algorithms that
    /// work on large portions of the grid may use this to look for performance gain. For
    /// example, [`Row::draw_copied`] uses this internally to call std's [`copy_from_slice`] when
    /// possible, which can be faster than manually copying elements one-by-one.
    ///
    /// [`copy_from_slice`]: https://doc.rust-lang.org/std/primitive.slice.html#method.copy_from_slice
    fn row_slice_mut(&mut self, y: usize) -> Option<&mut [Self::Item]>;

    /// Replace the value stored at `(x, y)` in the grid. If the provided coordinate was
    /// out of bounds, `None` is returned, otherwise the replaced value is returned.
    #[inline]
    fn set(&mut self, x: usize, y: usize, value: Self::Item) -> Option<Self::Item> {
        self.get_mut(x, y)
            .map(|curr| std::mem::replace(curr, value))
    }

    /// Replace the value stored at `(x, y)` in the grid, without bounds checking, and
    /// return the replaced value.
    ///
    /// For a safe alternative, see [`set`](Self::set).
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds coord is *[undefined behavior]*.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    unsafe fn set_unchecked(&mut self, x: usize, y: usize, value: Self::Item) -> Self::Item {
        std::mem::replace(self.get_unchecked_mut(x, y), value)
    }

    /// Get a mutable [`View`] into this grid, or `None` if the provided region is
    /// out of bounds.
    #[inline]
    fn try_view_mut(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) -> Option<View<&mut Self::Root>> {
        if x + w <= self.width() && y + h <= self.height() {
            let x = self.root_x() + x;
            let y = self.root_y() + y;
            Some(View::new(self.root_mut(), x, y, w, h))
        } else {
            None
        }
    }

    /// Get a mutable [`View`] into this grid. Panicks if the provided region is out
    /// of bounds.
    #[inline]
    fn view_mut(&mut self, x: usize, y: usize, w: usize, h: usize) -> View<&mut Self::Root> {
        self.try_view_mut(x, y, w, h)
            .expect("view does not overlap grid's bounds")
    }

    /// Mutably iterate over all values in the grid, with their positions.
    #[inline]
    fn iter_mut(&mut self) -> GridIter<&mut Self>
    where
        Self: Sized,
    {
        GridIter::new(self)
    }

    /// Mutably iterate over all columns in the grid.
    #[inline]
    fn cols_mut(&mut self) -> ColsIter<&mut Self>
    where
        Self: Sized,
    {
        ColsIter::new(self, self.width())
    }

    /// Return the column `x`, or `None` if `x` is out of bounds.
    #[inline]
    fn try_col_mut(&mut self, x: usize) -> Option<Col<&mut Self>> {
        (x < self.width()).then(|| Col::new(self, x))
    }

    /// Return the column `x`. Panics if `x` is out of bounds.
    #[inline]
    fn col_mut(&mut self, x: usize) -> Col<&mut Self> {
        self.try_col_mut(x).expect("column index out of bounds")
    }

    /// Mutably iterate over the rows of the grid.
    #[inline]
    fn rows_mut(&mut self) -> RowsIter<&mut Self>
    where
        Self: Sized,
    {
        RowsIter::new(self, self.height())
    }

    // Return the row `y`, or `None` if `y` is out of bounds.
    #[inline]
    fn try_row_mut(&mut self, y: usize) -> Option<Row<&mut Self>> {
        (y < self.height()).then(|| Row::new(self, y))
    }

    /// Return the row `y`. Panics if `y` is out of bounds.
    #[inline]
    fn row_mut(&mut self, y: usize) -> Row<&mut Self> {
        self.try_row_mut(y).expect("row index out of bounds")
    }

    /// Fill the entire grid with values provided by a function.
    #[inline]
    fn fill_with<F: FnMut() -> Self::Item>(&mut self, mut f: F)
    where
        Self: Sized,
    {
        for mut row in self.rows_mut() {
            row.fill_with(&mut f);
        }
    }

    /// Fill the entire grid with the provided value.
    #[inline]
    fn fill(&mut self, value: Self::Item)
    where
        Self: Sized,
        Self::Item: Clone,
    {
        let mut rows = self.rows_mut();
        if let Some(mut row) = rows.next() {
            for mut row in rows {
                row.fill(value.clone());
            }
            row.fill(value);
        }
    }

    /// Clone all values from a source grid into this one. Panics if the grids
    /// are not the same size.
    #[inline]
    fn draw_cloned<G2>(&mut self, grid: &G2)
    where
        G2: Grid<Item = Self::Item>,
        G2::Item: Clone,
        Self: Sized,
    {
        assert_eq!(self.width(), grid.width());
        assert_eq!(self.height(), grid.height());
        for (mut dst, src) in self.rows_mut().zip(grid.rows()) {
            dst.draw_cloned(src);
        }
    }

    /// Copy all values from a source grid into this one. Panics if the grids
    /// are not the same size.
    #[inline]
    fn draw_copied<G2>(&mut self, grid: &G2)
    where
        G2: Grid<Item = Self::Item>,
        G2::Item: Copy,
        Self: Sized,
    {
        assert_eq!(self.width(), grid.width());
        assert_eq!(self.height(), grid.height());
        for (mut dst, src) in self.rows_mut().zip(grid.rows()) {
            dst.draw_copied(src);
        }
    }
}

impl<T, const W: usize, const H: usize> GridMut for [[T; W]; H] {
    #[inline]
    fn root_mut(&mut self) -> &mut Self::Root {
        self
    }

    #[inline]
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item> {
        (x < W && y < H).then(|| &mut self[y][x])
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut Self::Item {
        self.as_mut_slice()
            .get_unchecked_mut(y)
            .get_unchecked_mut(x)
    }

    #[inline]
    fn row_slice_mut(&mut self, y: usize) -> Option<&mut [Self::Item]> {
        (y < H).then(|| self[y].as_mut_slice())
    }
}
