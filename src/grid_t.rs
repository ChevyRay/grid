use crate::{GridIndex, GridIter, GridIterMut, GridIterUnchecked, GridIterUncheckedMut};
use math::{int2, irect, Int2, IntRect};
use std::marker::PhantomData;

/// Trait for implementing a 2D grid of values generically over type `T`.
///
/// Implementing the required methods provides the custom grid type with
/// a bunch of algorithm implementations.
pub trait Grid<T> {
    /// Width of the grid (columns).
    fn width(&self) -> usize;

    /// Height of the grid (rows).
    fn height(&self) -> usize;

    /// Get a reference to the value stored at the position in the grid.
    ///
    /// ```rust
    /// # use grid::{Grid, VecGrid, int2};
    /// #
    /// let mut g = VecGrid::new(10, 10, ' ');
    /// g.set((0, 0), 'A');
    /// g.set((2, 1), 'B');
    /// g.set(int2(3, 4), 'C');
    ///
    /// assert_eq!(Some(&'A'), g.get(int2(0, 0)));
    /// assert_eq!(Some(&'B'), g.get(int2(2, 1)));
    /// assert_eq!(Some(&'C'), g.get((3, 4)));
    ///
    /// ```
    fn get<I: GridIndex<T>>(&self, index: I) -> Option<&T>;

    /// Get a mutable reference to the value stored at the position in the grid.
    ///
    /// ```rust
    /// # use grid::{Grid, VecGrid};
    /// #
    /// let mut g = VecGrid::new(10, 10, ' ');
    /// g.set((3, 2), 'A');
    ///
    /// assert_eq!(Some(&'A'), g.get((3, 2)));
    ///
    /// if let Some(val) = g.get_mut((3, 2)) {
    ///     *val = 'B';
    /// }
    ///
    /// assert_eq!(Some(&'B'), g.get((3, 2)));
    /// ```
    fn get_mut<I: GridIndex<T>>(&mut self, index: I) -> Option<&mut T>;

    /// Get an unchecked reference to the value stored at the position in the grid.
    ///
    /// ```rust
    /// # use grid::{Grid, VecGrid, int2};
    /// #
    /// let mut g = VecGrid::new(10, 10, ' ');
    /// g.set((0, 0), 'A');
    /// g.set((2, 1), 'B');
    /// g.set(int2(3, 4), 'C');
    ///
    /// unsafe {
    ///     assert_eq!(&'A', g.get_unchecked(int2(0, 0)));
    ///     assert_eq!(&'B', g.get_unchecked(int2(2, 1)));
    ///     assert_eq!(&'C', g.get_unchecked((3, 4)));
    /// }
    ///
    /// ```
    unsafe fn get_unchecked<I: GridIndex<T>>(&self, index: I) -> &T;

    /// Get an unchecked mutable reference to the value stored at the position in the grid.
    ///
    /// ```rust
    /// # use grid::{Grid, VecGrid};
    /// #
    /// let mut g = VecGrid::new(10, 10, ' ');
    /// g.set((3, 2), 'A');
    ///
    /// assert_eq!(Some(&'A'), g.get((3, 2)));
    ///
    /// unsafe { *g.get_unchecked_mut((3, 2)) = 'B' };
    ///
    /// assert_eq!(Some(&'B'), g.get((3, 2)));
    /// ```
    unsafe fn get_unchecked_mut<I: GridIndex<T>>(&mut self, index: I) -> &mut T;

    /// Sets the value at the position in the grid.
    ///
    /// ```rust
    /// # use grid::{Grid, VecGrid};
    /// #
    /// let mut g = VecGrid::<char>::new(10, 10, ' ');
    /// g.set((5, 4), 'X');
    /// ```
    fn set<I: GridIndex<T>, U: Into<T>>(&mut self, index: I, value: U) {
        if let Some(target) = self.get_mut(index) {
            *target = value.into();
        }
    }

    /// Sets the value at the position in the grid without bounds checking.
    ///
    /// If you know for certain that grid has a value stored at the position
    /// in the grid, you can use this to overwrite it and it will be faster
    /// than using regular [set](crate::Grid<T>::set), as this version will
    /// not bounds check the position.
    ///
    /// ```rust
    /// # use grid::{Grid, VecGrid};
    /// #
    /// let mut g = VecGrid::<char>::new(10, 10, ' ');
    /// unsafe { g.set_unchecked((5, 4), 'X') };
    /// ```
    unsafe fn set_unchecked<I: GridIndex<T>, U: Into<T>>(&mut self, index: I, value: U) {
        *self.get_unchecked_mut(index) = value.into();
    }

    /// Returns a rectangle that represents the entire area of the grid.
    fn bounds(&self) -> IntRect {
        irect(0, 0, self.width() as i32, self.height() as i32)
    }

    /// Fills the region of the grid with values provided by the supplied factory method.
    ///
    /// ```rust
    /// # use grid::{Grid, VecGrid, irect};
    /// #
    /// let mut g = VecGrid::<char>::new(10, 10, ' ');
    /// g.fill_with(irect(1, 1, 8, 8), |p| if p.x < 5 { 'L' } else { 'R' });
    ///
    /// assert_eq!(Some(&' '), g.get((0, 0)));
    /// assert_eq!(Some(&'L'), g.get((2, 2)));
    /// assert_eq!(Some(&'R'), g.get((6, 6)));
    /// ```
    fn fill_with<U: Into<T>, F: FnMut(Int2) -> U>(&mut self, rect: IntRect, mut f: F) {
        if let Some(rect) = self.bounds().overlap(&rect) {
            for p in rect.iter() {
                self.set(p, f(p).into())
            }
        }
    }

    /// Same as [fill_with](Grid<T>::fill_with), but with no bounds checking.
    unsafe fn fill_with_unchecked<U: Into<T>, F: FnMut(Int2) -> U>(
        &mut self,
        rect: IntRect,
        mut f: F,
    ) {
        if let Some(rect) = self.bounds().overlap(&rect) {
            for p in rect.iter() {
                self.set_unchecked(p, f(p).into())
            }
        }
    }

    /// Fills the region of the grid with the value provided.
    ///
    /// ```rust
    /// # use grid::{Grid, VecGrid, irect};
    /// #
    /// let mut g = VecGrid::<char>::new(10, 10, ' ');
    /// g.fill(irect(1, 1, 4, 8), 'L');
    /// g.fill(irect(5, 1, 4, 8), 'R');
    ///
    /// assert_eq!(Some(&' '), g.get((0, 0)));
    /// assert_eq!(Some(&'L'), g.get((2, 2)));
    /// assert_eq!(Some(&'R'), g.get((6, 6)));
    /// ```
    fn fill<U: Clone + Into<T>>(&mut self, rect: IntRect, value: U) {
        self.fill_with(rect, |_| value.clone());
    }

    /// Same as [fill](Grid<T>::fill), but with no bounds checking.
    unsafe fn fill_unchecked<U: Clone + Into<T>>(&mut self, rect: IntRect, value: U) {
        self.fill_with_unchecked(rect, |_| value.clone());
    }

    /// Clears the entire grid with values provided by the factory method.
    ///
    /// ```rust
    /// # use grid::{Grid, VecGrid};
    /// #
    /// let mut g = VecGrid::<i32>::new(5, 2, 0);
    /// g.clear_with(|p| p.x);
    /// assert_eq!(g.data(), &[
    ///     0, 1, 2, 3, 4,
    ///     0, 1, 2, 3, 4
    /// ]);
    /// ```
    fn clear_with<U: Into<T>, F: FnMut(Int2) -> U>(&mut self, f: F) {
        self.fill_with(self.bounds(), f);
    }

    /// Same as [clear_with](Grid<T>::clear_with), but with no bounds checking.
    unsafe fn clear_with_unchecked<U: Into<T>, F: FnMut(Int2) -> U>(&mut self, f: F) {
        self.fill_with_unchecked(self.bounds(), f);
    }

    /// Clears the entire grid with the value provided.
    ///
    /// ```rust
    /// # use grid::{Grid, VecGrid};
    /// #
    /// let mut g = VecGrid::<char>::new(2, 2, 'A');
    /// assert_eq!(g.data(), &['A', 'A', 'A', 'A']);
    /// g.clear('B');
    /// assert_eq!(g.data(), &['B', 'B', 'B', 'B']);
    /// ```
    fn clear<U: Clone + Into<T>>(&mut self, value: U) {
        self.fill_with(self.bounds(), |_| value.clone());
    }

    /// Same as [clear](Grid<T>::clear), but with no bounds checking.
    unsafe fn clear_unchecked<U: Clone + Into<T>>(&mut self, value: U) {
        self.fill_with_unchecked(self.bounds(), |_| value.clone());
    }

    /// Returns a rectangle that encloses all positions in the grid where values
    /// meet the provided condition.
    ///
    /// ```rust
    /// # use grid::{Grid, VecGrid, irect};
    /// #
    /// let mut g = VecGrid::<i32>::from_slice(4, 4, &[
    ///     0, 0, 0, 0,
    ///     0, 1, 2, 0,
    ///     0, 3, 4, 0,
    ///     0, 0, 0, 0,
    /// ]);
    ///
    /// assert_eq!(Some(irect(1, 1, 2, 2)), g.get_bounds(|&x| x > 0));
    /// assert_eq!(Some(irect(1, 2, 2, 1)), g.get_bounds(|&x| x >= 3));
    /// assert_eq!(Some(irect(0, 0, 4, 4)), g.get_bounds(|&x| x == 0));
    /// assert_eq!(None, g.get_bounds(|&x| x > 99));
    /// ```
    fn get_bounds<C: FnMut(&T) -> bool>(&self, mut cond: C) -> Option<IntRect> {
        let mut min = int2(i32::MAX, i32::MAX);
        let mut max = int2(i32::MIN, i32::MIN);
        for p in self.bounds().iter() {
            if let Some(val) = self.get(p) {
                if cond(val) {
                    min = min.min(p);
                    max = max.max(p);
                }
            }
        }
        (max.x >= min.x && max.y >= min.y)
            .then(|| irect(min.x, min.y, max.x - min.x + 1, max.y - min.y + 1))
    }

    /// Same as [get_bounds](Grid<T>::get_bounds), but with no bounds checking.
    unsafe fn get_bounds_unchecked<C: FnMut(&T) -> bool>(&self, mut cond: C) -> Option<IntRect> {
        let mut min = int2(i32::MAX, i32::MAX);
        let mut max = int2(i32::MIN, i32::MIN);
        for p in self.bounds().iter() {
            let val = self.get_unchecked(p);
            if cond(val) {
                min = min.min(p);
                max = max.max(p);
            }
        }
        (max.x >= min.x && max.y >= min.y)
            .then(|| irect(min.x, min.y, max.x - min.x + 1, max.y - min.y + 1))
    }

    /// Clone a region of values from one grid into this one.
    ///
    /// `grid`: the source grid to clone from
    ///
    /// `rect`: the source region to clone from
    ///
    /// `dest`: the top-left destination position to write the values to
    ///
    /// ```rust
    /// # use grid::{Grid, VecGrid, irect, int2};
    /// #
    /// let src = VecGrid::<char>::new(2, 2, 'X');
    /// let mut dst =  VecGrid::<char>::new(4, 4, ' ');
    ///
    /// dst.clone_rect(&src, src.bounds(), int2(1, 1));
    ///
    /// assert_eq!(dst.data(), &[
    ///     ' ', ' ', ' ', ' ',
    ///     ' ', 'X', 'X', ' ',
    ///     ' ', 'X', 'X', ' ',
    ///     ' ', ' ', ' ', ' ',
    /// ]);
    /// ```
    fn clone_rect<U: Clone + Into<T>, G: Grid<U>>(&mut self, grid: &G, rect: IntRect, dest: Int2) {
        if let Some(src_rect) = grid.bounds().overlap(&rect) {
            if let Some(dst_rect) = self.bounds().overlap(&(src_rect + dest)) {
                if let Some(src_rect) = grid.bounds().overlap(&(dst_rect - dest)) {
                    for p in src_rect.iter() {
                        if let Some(val) = grid.get(p) {
                            self.set(dest + p, val.clone());
                        }
                    }
                }
            }
        }
    }

    /// Same as [clone_rect](Grid<T>::clone_rect), but with no bounds checking.
    unsafe fn clone_rect_unchecked<U: Clone + Into<T>, G: Grid<U>>(
        &mut self,
        grid: &G,
        rect: IntRect,
        dest: Int2,
    ) {
        if let Some(src_rect) = grid.bounds().overlap(&rect) {
            if let Some(dst_rect) = self.bounds().overlap(&(src_rect + dest)) {
                if let Some(src_rect) = grid.bounds().overlap(&(dst_rect - dest)) {
                    for p in src_rect.iter() {
                        let val = grid.get_unchecked(p);
                        self.set_unchecked(dest + p, val.clone());
                    }
                }
            }
        }
    }

    /// Iterate over all values in the grid within the provided region.
    ///
    /// ```rust
    /// # use grid::{Grid, VecGrid, irect};
    /// #
    /// let mut g = VecGrid::<char>::from_slice(4, 4, &[
    ///     'A', 'B', 'C', 'D',
    ///     'E', 'F', 'G', 'H',
    ///     'I', 'J', 'K', 'L',
    ///     'M', 'N', 'O', 'P',
    /// ]);
    ///
    /// let chars: Vec<char> = g
    ///     .in_rect(irect(1, 1, 2, 2))
    ///     .map(|(pos, val)| *val)
    ///     .collect();
    ///
    /// assert_eq!(&chars, &['F', 'G', 'J', 'K']);
    /// ```
    fn in_rect(&self, rect: IntRect) -> GridIter<T, Self>
    where
        Self: Sized,
    {
        let rect = self
            .bounds()
            .overlap(&rect)
            .unwrap_or_else(|| IntRect::EMPTY);
        GridIter {
            grid: self,
            iter: rect.iter(),
            _marker: PhantomData::default(),
        }
    }

    /// Same as [in_rect](Grid<T>::in_rect), but with no bounds checking.
    fn in_rect_unchecked(&self, rect: IntRect) -> GridIterUnchecked<T, Self>
    where
        Self: Sized,
    {
        let rect = self
            .bounds()
            .overlap(&rect)
            .unwrap_or_else(|| IntRect::EMPTY);
        GridIterUnchecked {
            grid: self,
            iter: rect.iter(),
            _marker: PhantomData::default(),
        }
    }

    /// Iterate mutably over all values in the grid within the provided region.
    ///
    /// ```rust
    /// # use grid::{Grid, VecGrid, irect};
    /// #
    /// let mut g = VecGrid::<i32>::new(4, 4, 0);
    ///
    /// for (pos, val) in g.in_rect_mut(irect(1, 1, 2, 2)) {
    ///     *val = pos.x + pos.y;
    /// }
    ///
    /// assert_eq!(g.data(), &[
    ///     0, 0, 0, 0,
    ///     0, 2, 3, 0,
    ///     0, 3, 4, 0,
    ///     0, 0, 0, 0,
    /// ]);
    /// ```
    fn in_rect_mut(&mut self, rect: IntRect) -> GridIterMut<T, Self>
    where
        Self: Sized,
    {
        let rect = self
            .bounds()
            .overlap(&rect)
            .unwrap_or_else(|| IntRect::EMPTY);
        GridIterMut {
            grid: self,
            iter: rect.iter(),
            _marker: PhantomData::default(),
        }
    }

    /// Same as [in_rect_mut](Grid<T>::in_rect_mut), but with no bounds checking.
    unsafe fn in_rect_unchecked_mut(&mut self, rect: IntRect) -> GridIterUncheckedMut<T, Self>
    where
        Self: Sized,
    {
        let rect = self
            .bounds()
            .overlap(&rect)
            .unwrap_or_else(|| IntRect::EMPTY);
        GridIterUncheckedMut {
            grid: self,
            iter: rect.iter(),
            _marker: PhantomData::default(),
        }
    }

    /// Iterate over the entire grid. Equivalent to calling [in_rect](Grid<T>::in_rect)
    /// over the grid's [bounds](Grid<T>::bounds).
    fn iter(&self) -> GridIter<T, Self>
    where
        Self: Sized,
    {
        self.in_rect(self.bounds())
    }

    /// Same as [iter](Grid<T>::iter), but with no bounds checking.
    unsafe fn iter_unchecked(&self) -> GridIterUnchecked<T, Self>
    where
        Self: Sized,
    {
        self.in_rect_unchecked(self.bounds())
    }

    /// Iterate mutably over the entire grid. Equivalent to calling [in_rect_mut](Grid<T>::in_rect_mut)
    /// over the grid's [bounds](Grid<T>::bounds).
    fn iter_mut(&mut self) -> GridIterMut<T, Self>
    where
        Self: Sized,
    {
        self.in_rect_mut(self.bounds())
    }

    /// Same as [iter_mut](Grid<T>::iter_mut), but with no bounds checking.
    unsafe fn iter_unchecked_mut(&mut self) -> GridIterUncheckedMut<T, Self>
    where
        Self: Sized,
    {
        self.in_rect_unchecked_mut(self.bounds())
    }
}
