use crate::{Grid, GridIndex};

/// A fixed-size grid with data stored in a `[[T; W]; H]`. Rather than
/// living on the heap (like in [`VecGrid`](crate::VecGrid), the data for
/// this grid lives locally in the struct.
///
/// ```rust
/// use grid::{Grid, ConstGrid};
///
/// let mut g = ConstGrid::<char, 3, 3>::new(' ');
/// g.set((0, 0), 'A');
/// g.set((1, 1), 'B');
/// g.set((2, 2), 'C');
///
/// assert_eq!(g.data(), &[
///     ['A', ' ', ' '],
///     [' ', 'B', ' '],
///     [' ', ' ', 'C'],
/// ]);
/// ```
#[derive(Debug, Clone)]
pub struct ConstGrid<T, const W: usize, const H: usize> {
    data: [[T; W]; H],
}

impl<T: Default + Copy, const W: usize, const H: usize> Default for ConstGrid<T, W, H> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T, const W: usize, const H: usize> Grid<T> for ConstGrid<T, W, H> {
    fn width(&self) -> usize {
        W
    }

    fn height(&self) -> usize {
        H
    }

    fn get<I: GridIndex<T>>(&self, index: I) -> Option<&T> {
        let (x, y) = index.pos(W);
        self.data.get(y).and_then(|data| data.get(x))
    }

    fn get_mut<I: GridIndex<T>>(&mut self, index: I) -> Option<&mut T> {
        let (x, y) = index.pos(W);
        self.data.get_mut(y).and_then(|data| data.get_mut(x))
    }

    unsafe fn get_unchecked<I: GridIndex<T>>(&self, index: I) -> &T {
        let (x, y) = index.pos(W);
        self.data.get_unchecked(y).get_unchecked(x)
    }

    unsafe fn get_unchecked_mut<I: GridIndex<T>>(&mut self, index: I) -> &mut T {
        let (x, y) = index.pos(W);
        self.data.get_unchecked_mut(y).get_unchecked_mut(x)
    }
}

impl<T: Copy, const W: usize, const H: usize> ConstGrid<T, W, H> {
    /// Construct a new grid filled with the provided value.
    pub fn new<U: Into<T>>(fill: U) -> Self {
        Self {
            data: [[fill.into(); W]; H],
        }
    }
}

impl<T, const W: usize, const H: usize> ConstGrid<T, W, H> {
    // Get a linear slice of all the grid's values.
    pub fn data(&self) -> &[[T; W]; H] {
        &self.data
    }

    /// Get a mutable linear slice of all the grid's values.
    pub fn data_mut(&mut self) -> &mut [[T; W]; H] {
        &mut self.data
    }

    /// Flip the grid horizontally in-place, without copying,
    /// using [std::mem::swap](std::mem::swap) to move values.
    pub fn flip_x(&mut self) {
        if W > 0 {
            for x in 0..W {
                let x2 = (W - 1) - x;
                if x != x2 {
                    for y in 0..H {
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
        if H > 0 {
            for y in 0..H {
                let y2 = (H - 1) - y;
                if y != y2 {
                    for x in 0..W {
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

impl<T: Default + Copy, const W: usize, const H: usize> ConstGrid<T, W, H> {
    /// Rotate the grid 90º clockwise and write to the target grid. If no target
    /// is provided, a new one with the correct dimensions will be returned.
    pub fn rotate_right(&self, target: Option<ConstGrid<T, H, W>>) -> ConstGrid<T, H, W> {
        let mut target = target.unwrap_or_else(|| ConstGrid::<T, H, W>::default());
        let h = (H - 1) as i32;
        for (pos, val) in self.iter() {
            unsafe { target.set_unchecked((h - pos.y, pos.x), *val) };
        }
        target
    }

    /// Rotate the grid 90º counter-clockwise and write to the target grid. If no
    /// target is provided, a new one with the correct dimensions will be returned.
    pub fn rotate_left(&self, target: Option<ConstGrid<T, H, W>>) -> ConstGrid<T, H, W> {
        let mut target = target.unwrap_or_else(|| ConstGrid::<T, H, W>::default());
        let w = (W - 1) as i32;
        for (pos, val) in self.iter() {
            unsafe { target.set_unchecked((pos.y, w - pos.x), *val) };
        }
        target
    }
}
