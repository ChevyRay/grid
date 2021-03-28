use crate::{Grid, GridIndex};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub struct SparseGrid<T> {
    vals: Vec<T>,
    cols: Vec<i32>,
    row_inds: Vec<i32>,
    width: usize,
}

impl<T: Debug> Display for SparseGrid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "vals:     {:?}", self.vals)?;
        writeln!(f, "cols:     {:?}", self.cols)?;
        writeln!(f, "row_inds: {:?}", self.row_inds)?;
        writeln!(f, "size: {}, {}", self.width, self.height())
    }
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
}

impl<T> SparseGrid<T> {
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
}
