use crate::{Grid, GridIndex};

/// A resizable grid with data stored in a `Vec<T>`. This grid can be resized
/// at any time, and always has a value in every cell.
///
/// ```rust
/// use grid::{Grid, VecGrid};
///
/// // Create a 5x5 boolean grid
/// let mut g = VecGrid::<bool>::new(5, 5, false);
/// g.set((2, 1), true);
///
/// // Resize it to 10x10, filling all new slots with `true`
/// g.resize(10, 10, true);
/// g.set((7, 9), false);
/// ```
#[derive(Debug, Clone)]
pub struct VecGrid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Default for VecGrid<T> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            width: 0,
            height: 0,
        }
    }
}

impl<T> Grid<T> for VecGrid<T> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get<I: GridIndex<T>>(&self, index: I) -> Option<&T> {
        self.data.get(index.index(self.width()))
    }

    fn get_mut<I: GridIndex<T>>(&mut self, index: I) -> Option<&mut T> {
        let i = index.index(self.width());
        self.data.get_mut(i)
    }

    unsafe fn get_unchecked<I: GridIndex<T>>(&self, index: I) -> &T {
        self.data.get_unchecked(index.index(self.width()))
    }

    unsafe fn get_unchecked_mut<I: GridIndex<T>>(&mut self, index: I) -> &mut T {
        let index = index.index(self.width());
        self.data.get_unchecked_mut(index)
    }
}

impl<T> VecGrid<T> {
    /// Get a linear slice of all the grid's values.
    pub fn data(&self) -> &[T] {
        &self.data
    }

    /// Get a mutable linear slice of all the grid's values.
    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    /// Construct a new grid using the
    pub fn new_with<F: FnMut() -> T>(width: usize, height: usize, f: F) -> Self {
        let mut data = Vec::new();
        data.resize_with(width * height, f);
        Self {
            data,
            width,
            height,
        }
    }

    /// Construct a new grid with the dimensions, filled with the provided value.
    pub fn new<U: Clone + Into<T>>(width: usize, height: usize, value: U) -> Self {
        Self::new_with(width, height, || value.clone().into())
    }

    /// Construct a new grid with the provided dimensions, and its values cloned
    /// from the provided slice. If the slice is larger than the grid's internal
    /// array, the rest will be ignored. If the slice is smaller, then the remaining
    /// cells will be filled with `Default::default()`.
    pub fn from_slice<U: Default + Clone + Into<T>>(
        width: usize,
        height: usize,
        slice: &[U],
    ) -> Self {
        let count = width * height;
        let mut data = Vec::with_capacity(count);
        data.extend(slice.iter().take(count).map(|val| val.clone().into()));
        while data.len() < count {
            data.push(U::default().into());
        }
        Self {
            data,
            width,
            height,
        }
    }

    /// Resize the grid, filling any new empty cells with values provided
    /// by the supplied factory method.
    pub fn resize_with<F>(&mut self, width: usize, height: usize, f: F)
    where
        F: FnMut() -> T,
    {
        let width = width.max(0);
        let height = height.max(0);
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.data.resize_with((width * height) as usize, f);
        }
    }

    /// Resize the grid, filling any new empty cells with the provided value.
    pub fn resize<U: Clone + Into<T>>(&mut self, width: usize, height: usize, fill: U) {
        self.resize_with(width, height, || fill.clone().into());
    }

    /// Flip the grid horizontally in-place, without copying,
    /// using [std::mem::swap](std::mem::swap) to move values.
    pub fn flip_x(&mut self) {
        if self.width > 0 {
            let w = self.width as usize;
            let h = self.height as usize;
            for x in 0..w {
                let x2 = (w - 1) - x;
                if x != x2 {
                    for y in 0..h {
                        unsafe {
                            let a_ptr = self.get_unchecked_mut((x, y)) as *mut T;
                            let b_ptr = self.get_unchecked_mut((x2, y)) as *mut T;
                            std::mem::swap(&mut *a_ptr, &mut *b_ptr);
                        }
                    }
                }
            }
        }
    }

    /// Flip the grid vertically in-place, without copying,
    /// using [std::mem::swap](std::mem::swap) to move values.
    pub fn flip_y(&mut self) {
        if self.height > 0 {
            let w = self.width as usize;
            let h = self.height as usize;
            for y in 0..h {
                let y2 = (h - 1) - y;
                if y != y2 {
                    for x in 0..w {
                        unsafe {
                            let a_ptr = self.get_unchecked_mut((x, y)) as *mut T;
                            let b_ptr = self.get_unchecked_mut((x, y2)) as *mut T;
                            std::mem::swap(&mut *a_ptr, &mut *b_ptr);
                        }
                    }
                }
            }
        }
    }
}

impl<T: Clone + Default> VecGrid<T> {
    fn get_target(target: Option<Self>, w: usize, h: usize) -> Self {
        target
            .and_then(|mut grid| {
                grid.resize(w, h, T::default());
                Some(grid)
            })
            .unwrap_or_else(|| Self::new(w, h, T::default()))
    }

    /// Rotate the grid 90º clockwise and write to the target grid. If no target
    /// is provided, a new one with the correct dimensions will be returned.
    ///
    /// ```rust
    /// use grid::{Grid, VecGrid};
    ///
    /// let orig = VecGrid::<char>::from_slice(3, 2, &[
    ///     'A', 'B', 'C',
    ///     'D', 'E', 'F',
    /// ]);
    ///
    /// let rotated = orig.rotate_right(None);
    ///
    /// assert_eq!(orig.width(), rotated.height());
    /// assert_eq!(orig.height(), rotated.width());
    /// assert_eq!(rotated.data(), &[
    ///     'D', 'A',
    ///     'E', 'B',
    ///     'F', 'C',
    /// ]);
    /// ```
    pub fn rotate_right(&self, target: Option<Self>) -> Self {
        let mut target = Self::get_target(target, self.height, self.width);
        let h = (self.height - 1) as i32;
        for (pos, val) in self.iter() {
            unsafe { target.set_unchecked((h - pos.y, pos.x), val.clone()) };
        }
        target
    }

    /// Rotate the grid 90º counter-clockwise and write to the target grid. If no
    /// target is provided, a new one with the correct dimensions will be returned.
    ///
    /// ```rust
    /// use grid::{Grid, VecGrid};
    ///
    /// let orig = VecGrid::<char>::from_slice(3, 2, &[
    ///     'A', 'B', 'C',
    ///     'D', 'E', 'F',
    /// ]);
    ///
    /// let rotated = orig.rotate_left(None);
    ///
    /// assert_eq!(orig.width(), rotated.height());
    /// assert_eq!(orig.height(), rotated.width());
    /// assert_eq!(rotated.data(), &[
    ///     'C', 'F',
    ///     'B', 'E',
    ///     'A', 'D',
    /// ]);
    /// ```
    pub fn rotate_left(&self, target: Option<Self>) -> Self {
        let mut target = Self::get_target(target, self.height, self.width);
        let w = (self.width - 1) as i32;
        for (pos, val) in self.iter() {
            unsafe { target.set_unchecked((pos.y, w - pos.x), val.clone()) };
        }
        target
    }
}
