use std::ops::{Deref, DerefMut};

pub struct Grid<T> {
    width: usize,
    height: usize,
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
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn new_with<F>(width: usize, height: usize, f: F) -> Self
    where
        F: FnMut() -> T,
    {
        let mut grid = Grid::default();
        grid.resize_with(width, height, f);
        grid
    }

    pub fn resize_with<F>(&mut self, width: usize, height: usize, f: F)
    where
        F: FnMut() -> T,
    {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.data.resize_with(width * height, f);
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y * self.width + x)
    }

    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> &T {
        self.data.get_unchecked(y * self.width + x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data.get_mut(y * self.width + x)
    }

    pub unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.data.get_unchecked_mut(y * self.width + x)
    }
}

impl<T> Grid<T>
where
    T: Default,
{
    pub fn new_with_default(width: usize, height: usize) -> Self {
        let mut grid = Grid::default();
        grid.resize_with_default(width, height);
        grid
    }

    pub fn resize_with_default(&mut self, width: usize, height: usize) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.data.resize_with(width * height, || T::default());
        }
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(width: usize, height: usize, fill: T) -> Self {
        let mut grid = Grid::default();
        grid.resize(width, height, fill);
        grid
    }

    pub fn resize(&mut self, width: usize, height: usize, fill: T) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.data.resize(width * height, fill);
        }
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
