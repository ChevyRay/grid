use crate::cols_iter::ColsIter;
use crate::{Col, Grid, GridIterMut, Row, RowsIter, View};

pub trait GridMut: Grid {
    type RootMut: GridMut<Item = Self::Item>;

    fn root_mut(&mut self) -> &mut Self::RootMut;
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item>;

    /// # Safety
    ///
    /// Calling this method with an out-of-bounds coord is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut Self::Item;

    fn row_slice_mut(&mut self, y: usize) -> Option<&mut [Self::Item]>;

    #[inline]
    fn set(&mut self, x: usize, y: usize, value: Self::Item) -> Option<Self::Item> {
        self.get_mut(x, y)
            .map(|curr| std::mem::replace(curr, value))
    }

    /// # Safety
    ///
    /// Calling this method with an out-of-bounds coord is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
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
    ) -> Option<View<&mut Self::RootMut>> {
        if x + w <= self.width() && y + h <= self.height() {
            let x = self.root_x() + x;
            let y = self.root_y() + y;
            Some(View::new(self.root_mut(), x, y, w, h))
        } else {
            None
        }
    }

    #[inline]
    fn view_mut(&mut self, x: usize, y: usize, w: usize, h: usize) -> View<&mut Self::RootMut> {
        self.try_view_mut(x, y, w, h)
            .expect("view does not overlap grid's bounds")
    }

    #[inline]
    fn full_view_mut(&mut self) -> View<&mut Self::RootMut> {
        self.view_mut(0, 0, self.width(), self.height())
    }

    #[inline]
    fn iter_mut(&mut self) -> GridIterMut<'_, Self>
    where
        Self: Sized,
    {
        GridIterMut::new(self)
    }

    #[inline]
    fn cols_mut(&mut self) -> ColsIter<&mut Self>
    where
        Self: Sized,
    {
        ColsIter::new(self, self.width())
    }

    #[inline]
    fn try_col_mut(&mut self, x: usize) -> Option<Col<&mut Self>> {
        (x < self.width()).then(|| Col::new(self, x))
    }

    #[inline]
    fn col_mut(&mut self, x: usize) -> Col<&mut Self> {
        self.try_col_mut(x).expect("column index out of bounds")
    }

    #[inline]
    fn rows_mut(&mut self) -> RowsIter<&mut Self>
    where
        Self: Sized,
    {
        RowsIter::new(self, self.height())
    }

    #[inline]
    fn try_row_mut(&mut self, y: usize) -> Option<Row<&mut Self>> {
        (y < self.height()).then(|| Row::new(self, y))
    }

    #[inline]
    fn row_mut(&mut self, y: usize) -> Row<&mut Self> {
        self.try_row_mut(y).expect("row index out of bounds")
    }

    #[inline]
    fn fill_with<F: FnMut() -> Self::Item>(&mut self, mut f: F)
    where
        Self: Sized,
    {
        for mut row in self.rows_mut() {
            row.fill_with(&mut f);
        }
    }

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

    #[inline]
    fn clone_from<G2>(&mut self, grid: &G2)
    where
        G2: Grid<Item = Self::Item>,
        G2::Item: Clone,
        Self: Sized,
    {
        for (mut dst, src) in self.rows_mut().zip(grid.rows()) {
            dst.clone_from(src);
        }
    }

    #[inline]
    fn copy_from<G2>(&mut self, grid: &G2)
    where
        G2: Grid<Item = Self::Item>,
        G2::Item: Copy,
        Self: Sized,
    {
        for (mut dst, src) in self.rows_mut().zip(grid.rows()) {
            dst.copy_from(src);
        }
    }
}

impl<T, const W: usize, const H: usize> GridMut for [[T; W]; H] {
    type RootMut = Self;

    #[inline]
    fn root_mut(&mut self) -> &mut Self::RootMut {
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
