use crate::{Grid, GridMut};
use std::marker::PhantomData;

pub struct GridBuf<T, S = Vec<T>> {
    width: usize,
    height: usize,
    store: S,
    marker: PhantomData<T>,
}

impl<T, S> GridBuf<T, S> {
    #[inline]
    pub fn with_store(width: usize, height: usize, store: S) -> Self
    where
        S: AsRef<[T]>,
    {
        let len = width.checked_mul(height).expect("grid capacity overflow");
        assert_eq!(len, store.as_ref().len());
        Self {
            width,
            height,
            store,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T]
    where
        S: AsRef<[T]>,
    {
        self.store.as_ref()
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T]
    where
        S: AsMut<[T]>,
    {
        self.store.as_mut()
    }
}

impl<T> GridBuf<T, Vec<T>> {
    #[inline]
    pub fn new_with<F: FnMut() -> T>(width: usize, height: usize, fill: F) -> Self {
        let len = width.checked_mul(height).expect("grid capacity overflow");
        let mut store = Vec::new();
        store.resize_with(len, fill);
        Self {
            width,
            height,
            store,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self::new_with(width, height, T::default)
    }
}

impl<T, S: AsRef<[T]>> Grid<T> for GridBuf<T, S> {
    type Root = Self;

    #[inline]
    fn root(&self) -> &Self::Root {
        self
    }

    #[inline]
    fn width(&self) -> usize {
        self.width
    }

    #[inline]
    fn height(&self) -> usize {
        self.height
    }

    #[inline]
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        y.checked_mul(self.width)
            .and_then(|y| y.checked_add(x))
            .and_then(|i| self.as_slice().get(i))
    }

    #[inline]
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &T {
        let i = y.unchecked_mul(self.width).unchecked_add(x);
        self.as_slice().get_unchecked(i)
    }

    #[inline]
    fn row_slice(&self, y: usize) -> Option<&[T]> {
        y.checked_mul(self.width)
            .and_then(|i| self.as_slice().get(i..(i + self.width)))
    }
}

impl<T, S: AsRef<[T]> + AsMut<[T]>> GridMut<T> for GridBuf<T, S> {
    #[inline]
    fn root_mut(&mut self) -> &mut Self::Root {
        self
    }

    #[inline]
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        y.checked_mul(self.width)
            .and_then(|y| y.checked_add(x))
            .and_then(|i| self.as_mut_slice().get_mut(i))
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut T {
        let i = y.unchecked_mul(self.width).unchecked_add(x);
        self.as_mut_slice().get_unchecked_mut(i)
    }

    #[inline]
    fn row_slice_mut(&mut self, y: usize) -> Option<&mut [T]> {
        let w = self.width;
        y.checked_mul(w)
            .and_then(|i| self.as_mut_slice().get_mut(i..(i + w)))
    }
}
