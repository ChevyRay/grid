use crate::{Grid, GridIndex};
use std::fmt::Debug;

/// A flexible-sized grid which only stores the values you set in it.
/// This is useful for very large grids that contain lots of empty
/// space, but you still want the ergonomics of a 2D grid structure.
///
/// You can insert items outside the bounds of this grid type and the
/// grid will flex to fit it.
///
/// ```rust
/// use grid::{Grid, SparseGrid};
///
/// let mut g = SparseGrid::<char>::default();
/// g.set((1, 1), 'A');
/// g.set((2, 2), 'B');
/// g.set((3, 3), 'C');
///
/// assert_eq!(g.width(), 4);
/// assert_eq!(g.height(), 4);
/// assert_eq!(g.get((1, 1)), Some(&'A'));
///
/// let chars: Vec<char> = g
///     .iter()
///     .map(|(_, val)| *val)
///     .collect();
///
/// assert_eq!(&chars, &['A', 'B', 'C']);
/// ```
/// Notice how in this example, iterating over the grid only yields the
/// cells that were actually assigned. Cells that aren't being used are
/// not allocated, unlike in [`VecGrid`](crate::VecGrid) and [`ConstGrid`](crate::ConstGrid)
///
///
/// # TODO
///
/// Add support for item removal.
#[derive(Debug, Clone)]
pub struct SparseGrid<T> {
    vals: Vec<T>,
    cols: Vec<i32>,
    row_inds: Vec<i32>,
    width: usize,
}

impl<T> Default for SparseGrid<T> {
    fn default() -> Self {
        Self {
            vals: Vec::new(),
            cols: Vec::new(),
            row_inds: vec![0],
            width: 0,
        }
    }
}

impl<T> Grid<T> for SparseGrid<T> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.row_inds.len() - 1
    }

    fn get<I: GridIndex<T>>(&self, index: I) -> Option<&T> {
        let (x, y) = index.pos(self.width);
        if y < self.height() {
            let row_i = unsafe { *self.row_inds.get_unchecked(y) as usize };
            let row_j = unsafe { *self.row_inds.get_unchecked(y + 1) as usize };
            if let Ok(ind) = self.cols[row_i..row_j].binary_search(&(x as i32)) {
                return Some(unsafe { self.vals.get_unchecked(row_i + ind) });
            }
        }
        None
    }

    fn get_mut<I: GridIndex<T>>(&mut self, index: I) -> Option<&mut T> {
        let (x, y) = index.pos(self.width);
        if y < self.height() {
            let row_i = unsafe { *self.row_inds.get_unchecked(y) as usize };
            let row_j = unsafe { *self.row_inds.get_unchecked(y + 1) as usize };
            if let Ok(ind) = self.cols[row_i..row_j].binary_search(&(x as i32)) {
                return Some(unsafe { self.vals.get_unchecked_mut(row_i + ind) });
            }
        }
        None
    }

    unsafe fn get_unchecked<I: GridIndex<T>>(&self, index: I) -> &T {
        let (x, y) = index.pos(self.width);
        let row_i = *self.row_inds.get_unchecked(y) as usize;
        let row_j = *self.row_inds.get_unchecked(y + 1) as usize;
        if let Ok(ind) = self.cols[row_i..row_j].binary_search(&(x as i32)) {
            self.vals.get_unchecked(row_i + ind)
        } else {
            panic!("no value in SparseGrid at ({}, {})", x, y)
        }
    }

    unsafe fn get_unchecked_mut<I: GridIndex<T>>(&mut self, index: I) -> &mut T {
        let (x, y) = index.pos(self.width);
        let row_i = *self.row_inds.get_unchecked(y) as usize;
        let row_j = *self.row_inds.get_unchecked(y + 1) as usize;
        if let Ok(ind) = self.cols[row_i..row_j].binary_search(&(x as i32)) {
            self.vals.get_unchecked_mut(row_i + ind)
        } else {
            panic!("no value in SparseGrid at ({}, {})", x, y)
        }
    }

    fn set<I: GridIndex<T>, U: Into<T>>(&mut self, index: I, value: U) {
        let (x, y) = index.pos(self.width);
        let top = *self.row_inds.last().unwrap();
        while y >= self.height() {
            self.row_inds.push(top);
        }
        let row_i = unsafe { *self.row_inds.get_unchecked(y) as usize };
        let row_j = unsafe { *self.row_inds.get_unchecked(y + 1) as usize };
        match self.cols[row_i..row_j].binary_search(&(x as i32)) {
            Ok(ind) => unsafe { *self.vals.get_unchecked_mut(row_i + ind) = value.into() },
            Err(ind) => {
                let ind = row_i + ind;
                self.cols.insert(ind, x as i32);
                self.vals.insert(ind, value.into());
                self.width = self.width.max(x + 1);

                // Every row after the one you inserted at starts one index later now
                for row_ind in &mut self.row_inds[(y + 1)..] {
                    *row_ind += 1;
                }
            }
        }
    }
}

impl<T> SparseGrid<T> {
    /// Constructs a new empty grid with the provided size. This does not
    /// allocate or construct any `T` values, it merely prepares the data
    /// structure for faster insertion.
    pub fn new(width: usize, height: usize) -> Self {
        let mut row_inds = Vec::new();
        row_inds.resize(height + 1, 0);
        Self {
            vals: Vec::new(),
            cols: Vec::new(),
            row_inds,
            width,
        }
    }

    /// A linear slice of all the grid's internal values.
    pub fn values(&self) -> &[T] {
        &self.vals
    }

    /// A linear mutable slice of all the grid's internal values.
    pub fn values_mut(&mut self) -> &mut [T] {
        &mut self.vals
    }
}
