use crate::{Grid, GridIndex};
use math::{int2, irect, Int2, IntRect, IntRectIter};
use std::marker::PhantomData;

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

    fn fill_with<U: Into<T>, F: FnMut(Int2) -> U>(&mut self, rect: IntRect, mut f: F) {
        if let Some(rect) = self.bounds().overlap(&rect) {
            for p in rect.iter() {
                unsafe { self.set_unchecked(p, f(p).into()) };
            }
        }
    }

    fn get_bounds<C: FnMut(&T) -> bool>(&self, mut cond: C) -> Option<IntRect> {
        let mut min = int2(i32::MIN, i32::MIN);
        let mut max = int2(i32::MAX, i32::MAX);
        for p in self.bounds().iter() {
            let val = unsafe { self.get_unchecked(p) };
            if cond(val) {
                min = min.min(p);
                max = max.max(p);
            }
        }
        (max.x > min.x && max.y > min.y).then(|| irect(min.x, min.y, max.x - min.x, max.y - min.y))
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

    pub unsafe fn get_unchecked<I: GridIndex<T>>(&self, index: I) -> &T {
        self.data.get_unchecked(index.index(self.width))
    }

    pub unsafe fn get_unchecked_mut<I: GridIndex<T>>(&mut self, index: I) -> &mut T {
        self.data.get_unchecked_mut(index.index(self.width))
    }

    pub unsafe fn set_unchecked<I: GridIndex<T>, U: Into<T>>(&mut self, index: I, value: U) {
        *self.get_unchecked_mut(index) = value.into();
    }

    pub fn flip_x(&mut self) {
        if self.width > 0 {
            let w = self.width as usize;
            let h = self.height as usize;
            for x in 0..w {
                let x2 = (w - 1) - x;
                if x != x2 {
                    for y in 0..h {
                        let a_ptr = &mut self.data[y * w + x] as *mut T;
                        let b_ptr = &mut self.data[y * w + x2] as *mut T;
                        unsafe {
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
                        let a_ptr = &mut self.data[y * w + x] as *mut T;
                        let b_ptr = &mut self.data[y2 * w + x] as *mut T;
                        unsafe {
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

    pub fn in_rect_unchecked(&self, rect: IntRect) -> VecGridIter<T> {
        let rect = self
            .bounds()
            .overlap(&rect)
            .unwrap_or_else(|| IntRect::EMPTY);
        VecGridIter {
            grid: self,
            iter: rect.iter(),
            _marker: PhantomData::default(),
        }
    }

    pub fn in_rect_unchecked_mut(&mut self, rect: IntRect) -> VecGridIterMut<T> {
        let rect = self
            .bounds()
            .overlap(&rect)
            .unwrap_or_else(|| IntRect::EMPTY);
        VecGridIterMut {
            grid: self,
            iter: rect.iter(),
            _marker: PhantomData::default(),
        }
    }

    pub fn iter_unchecked(&self) -> VecGridIter<T> {
        self.in_rect_unchecked(self.bounds())
    }

    pub fn iter_unchecked_mut(&mut self) -> VecGridIterMut<T> {
        self.in_rect_unchecked_mut(self.bounds())
    }
}

pub struct VecGridIter<'a, T> {
    grid: &'a VecGrid<T>,
    iter: IntRectIter,
    _marker: PhantomData<T>,
}

impl<'a, T: 'a> Iterator for VecGridIter<'a, T> {
    type Item = (Int2, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|pos| {
            let val = unsafe { self.grid.get_unchecked(pos) };
            Some((pos, val))
        })
    }
}

pub struct VecGridIterMut<'a, T> {
    grid: &'a mut VecGrid<T>,
    iter: IntRectIter,
    _marker: PhantomData<T>,
}

impl<'a, T: 'a> Iterator for VecGridIterMut<'a, T> {
    type Item = (Int2, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|pos| {
            let val = unsafe { self.grid.get_unchecked_mut(pos) };
            let ptr: *mut T = val;
            unsafe { Some((pos, &mut *ptr)) }
        })
    }
}
