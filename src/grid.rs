use crate::{GridIter, GridIterMut, GridValues, GridValuesMut};
use math::{int2, irect, Int2, IntRect};
use std::ops::{Deref, DerefMut};

pub struct Grid<T> {
    width: i32,
    height: i32,
    data: Vec<T>,
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            width: 0,
            height: 0,
        }
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn new_with<F>(width: i32, height: i32, f: F) -> Self
    where
        F: FnMut() -> T,
    {
        let mut grid = Grid::default();
        grid.resize_with(width, height, f);
        grid
    }

    pub fn from_iter<I>(width: i32, height: i32, mut iter: I) -> Option<Self>
    where
        I: Iterator<Item = T>,
    {
        let size = (width * height) as usize;
        let mut data = Vec::with_capacity(size);
        while data.len() < size {
            data.push(iter.next()?);
        }
        Some(Self {
            width,
            height,
            data,
        })
    }

    pub fn resize_with<F>(&mut self, width: i32, height: i32, f: F)
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

    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        self.contains(x, y)
            .then(|| unsafe { self.data.get_unchecked((y * self.width + x) as usize) })
    }

    pub fn getp(&self, p: Int2) -> Option<&T> {
        self.get(p.x, p.y)
    }

    pub unsafe fn get_unchecked(&self, x: i32, y: i32) -> &T {
        self.data.get_unchecked((y * self.width + x) as usize)
    }

    pub unsafe fn getp_unchecked(&self, p: Int2) -> &T {
        self.get_unchecked(p.x, p.y)
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        if self.contains(x, y) {
            let val = unsafe { self.data.get_unchecked_mut((y * self.width + x) as usize) };
            Some(val)
        } else {
            None
        }
    }

    pub fn getp_mut(&mut self, p: Int2) -> Option<&mut T> {
        self.get_mut(p.x, p.y)
    }

    pub unsafe fn get_unchecked_mut(&mut self, x: i32, y: i32) -> &mut T {
        self.data.get_unchecked_mut((y * self.width + x) as usize)
    }

    pub unsafe fn getp_unchecked_mut(&mut self, p: Int2) -> &mut T {
        self.get_unchecked_mut(p.x, p.y)
    }

    pub fn set(&mut self, x: i32, y: i32, value: T) {
        if self.contains(x, y) {
            unsafe {
                self.set_unchecked(x, y, value);
            }
        }
    }

    pub fn setp(&mut self, p: Int2, value: T) {
        self.set(p.x, p.y, value);
    }

    pub unsafe fn set_unchecked(&mut self, x: i32, y: i32, value: T) {
        *self.get_unchecked_mut(x, y) = value;
    }

    pub unsafe fn setp_unchecked(&mut self, p: Int2, value: T) {
        self.set_unchecked(p.x, p.y, value);
    }

    pub fn set_rect_with<F>(&mut self, rect: &IntRect, mut f: F)
    where
        F: FnMut(i32, i32) -> T,
    {
        if let Some(rect) = self.bounds().overlap(&rect) {
            for y in rect.y..rect.bottom() {
                for x in rect.x..rect.right() {
                    unsafe {
                        *self.get_unchecked_mut(x, y) = f(x, y);
                    }
                }
            }
        }
    }

    pub fn clear_with<F>(&mut self, f: F)
    where
        F: FnMut(i32, i32) -> T,
    {
        self.set_rect_with(&self.bounds(), f);
    }

    pub fn copy_from<'a, U>(&mut self, other: &'a Grid<U>, src_rect: &IntRect, dst_pos: Int2)
    where
        &'a U: Into<T>,
    {
        let rect = irect(dst_pos.x, dst_pos.y, src_rect.w, src_rect.h);
        if let Some(rect) = rect.overlap(&src_rect) {
            self.set_rect_with(&rect, |x, y| unsafe { other.get_unchecked(x, y).into() })
        }
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        self.bounds().contains(int2(x, y))
    }

    pub fn bounds(&self) -> IntRect {
        irect(0, 0, self.width, self.height)
    }

    pub fn get_bounds<C>(&self, mut cond: C) -> Option<IntRect>
    where
        C: FnMut(&T) -> bool,
    {
        let mut min = int2(i32::MIN, i32::MIN);
        let mut max = int2(i32::MAX, i32::MAX);
        for y in 0..self.height {
            for x in 0..self.width {
                let val = unsafe { self.get_unchecked(x, y) };
                if cond(val) {
                    min = min.min(int2(x, y));
                    max = max.max(int2(x, y));
                }
            }
        }
        (max.x > min.x && max.y > min.y).then(|| irect(min.x, min.y, max.x - min.x, max.y - min.y))
    }

    pub fn iter(&self) -> GridIter<T> {
        GridIter {
            grid: self,
            iter: self.bounds().iter(),
        }
    }

    pub fn iter_mut(&mut self) -> GridIterMut<T> {
        let iter = self.bounds().iter();
        GridIterMut { grid: self, iter }
    }

    pub fn values(&self) -> GridValues<T> {
        let iter = self.bounds().iter();
        GridValues { grid: self, iter }
    }

    pub fn values_mut(&mut self) -> GridValuesMut<T> {
        let iter = self.bounds().iter();
        GridValuesMut { grid: self, iter }
    }

    pub fn rect(&self, rect: &IntRect) -> Option<GridIter<T>> {
        let iter = self.bounds().overlap(rect)?.iter();
        Some(GridIter { grid: self, iter })
    }

    pub fn rect_mut(&mut self, rect: &IntRect) -> Option<GridIterMut<T>> {
        let iter = self.bounds().overlap(rect)?.iter();
        Some(GridIterMut { grid: self, iter })
    }

    pub fn rect_values(&self, rect: &IntRect) -> Option<GridValues<T>> {
        let iter = self.bounds().overlap(rect)?.iter();
        Some(GridValues { grid: self, iter })
    }

    pub fn rect_values_mut(&mut self, rect: &IntRect) -> Option<GridValuesMut<T>> {
        let iter = self.bounds().overlap(rect)?.iter();
        Some(GridValuesMut { grid: self, iter })
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(width: i32, height: i32, fill: T) -> Self {
        let mut grid = Grid::default();
        grid.resize(width, height, fill);
        grid
    }

    pub fn resize(&mut self, width: i32, height: i32, fill: T) {
        let width = width.max(0);
        let height = height.max(0);
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.data.resize((width * height) as usize, fill);
        }
    }

    pub fn set_rect(&mut self, rect: &IntRect, value: T) {
        if let Some(rect) = self.bounds().overlap(&rect) {
            for y in rect.y..rect.bottom() {
                for x in rect.x..rect.right() {
                    unsafe {
                        *self.get_unchecked_mut(x, y) = value.clone();
                    }
                }
            }
        }
    }

    pub fn clear(&mut self, value: T) {
        self.set_rect(&self.bounds(), value);
    }
}

impl<T> AsRef<[T]> for Grid<T> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

impl<T> AsMut<[T]> for Grid<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<T> Deref for Grid<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
