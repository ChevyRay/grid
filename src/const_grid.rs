use crate::{Grid, GridIndex};

pub struct ConstGrid<T, const W: usize, const H: usize> {
    data: [[T; W]; H],
}

impl<T: Default + Copy, const W: usize, const H: usize> Default for ConstGrid<T, W, H> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Copy, const W: usize, const H: usize> ConstGrid<T, W, H> {
    pub fn new<U: Into<T>>(fill: U) -> Self {
        Self {
            data: [[fill.into(); W]; H],
        }
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

impl<T, const W: usize, const H: usize> ConstGrid<T, W, H> {
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
