use crate::{Grid, GridIndex};

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
    pub fn new_with<F: FnMut() -> T>(width: usize, height: usize, f: F) -> Self {
        let mut data = Vec::new();
        data.resize_with(width * height, f);
        Self {
            data,
            width,
            height,
        }
    }

    pub fn new<U: Clone + Into<T>>(width: usize, height: usize, value: U) -> Self {
        Self::new_with(width, height, || value.clone().into())
    }

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

    pub fn resize<U: Clone + Into<T>>(&mut self, width: usize, height: usize, fill: U) {
        self.resize_with(width, height, || fill.clone().into());
    }

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

    pub fn rotate_right(&self, target: Option<Self>) -> Self {
        let mut target = Self::get_target(target, self.height, self.width);
        let h = (self.height - 1) as i32;
        for (pos, val) in self.iter() {
            target.set((h - pos.y, pos.x), val.clone());
        }
        target
    }

    pub fn rotate_left(&self, target: Option<Self>) -> Self {
        let mut target = Self::get_target(target, self.height, self.width);
        let w = (self.width - 1) as i32;
        for (pos, val) in self.iter() {
            target.set((pos.y, w - pos.x), val.clone());
        }
        target
    }
}
