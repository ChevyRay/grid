use crate::{Coord, CoordComponent, Grid, GridIter, GridMut};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

/// A grid implementation for different storage types.
pub struct GridBuf<T, S = Vec<T>> {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) store: S,
    pub(crate) marker: PhantomData<T>,
}

/// A grid implementation using a `Vec` for storage.
pub type VecGrid<T> = GridBuf<T, Vec<T>>;

/// A grid implementation using an array for storage.
pub type ArrGrid<T, const N: usize> = GridBuf<T, [T; N]>;

/// A grid implementation using a slice for storage.
pub type SliceGrid<'a, T> = GridBuf<T, &'a [T]>;

impl<T, S> GridBuf<T, S> {
    /// Create a new grid using the provided storage. Panics if the length
    /// of `store` is not equal to `width * height`.
    #[inline]
    pub fn with_store(width: usize, height: usize, store: S) -> Self
    where
        S: AsRef<[T]>,
    {
        assert_eq!(width.checked_mul(height), Some(store.as_ref().len()));
        Self {
            width,
            height,
            store,
            marker: PhantomData,
        }
    }

    /// Get the contents of the grid as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[T]
    where
        S: AsRef<[T]>,
    {
        self.store.as_ref()
    }

    /// Get the contents of the grid as a mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T]
    where
        S: AsMut<[T]>,
    {
        self.store.as_mut()
    }

    /// Drop the grid and return its storage.
    #[inline]
    pub fn to_store(self) -> S {
        self.store
    }
}

impl<T> VecGrid<T> {
    /// Create a new `VecGrid` filled with values from the provided function.
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

    /// Create a new `VecGrid` fill with default values.
    #[inline]
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self::new_with(width, height, T::default)
    }
}

impl<'a, T> SliceGrid<'a, T> {
    /// Create a new `SliceGrid` from the provided slice. Panics if the length of
    /// the slice is not exactly `width * height`.
    #[inline]
    pub fn new(width: usize, height: usize, slice: &'a [T]) -> Self {
        Self::with_store(width, height, slice)
    }
}

impl<T, const N: usize> ArrGrid<T, N> {
    /// Create a new `ArrGrid` filled with values from the provided function.
    #[inline]
    pub fn new_with<F: FnMut() -> T>(width: usize, height: usize, mut fill: F) -> Self {
        Self::with_store(width, height, std::array::from_fn(|_| fill()))
    }

    /// Create a new `ArrGrid` fill with default values.
    #[inline]
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self::new_with(width, height, T::default)
    }
}

impl<T, S: AsRef<[T]>> Grid for GridBuf<T, S> {
    type Item = T;
    type Root = Self;

    #[inline]
    fn root(&self) -> &Self::Root {
        self
    }

    #[inline]
    fn root_x(&self) -> usize {
        0
    }

    #[inline]
    fn root_y(&self) -> usize {
        0
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
    fn get(&self, x: usize, y: usize) -> Option<&Self::Item> {
        y.checked_mul(self.width)
            .and_then(|y| y.checked_add(x))
            .and_then(|i| self.as_slice().get(i))
    }

    #[inline]
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Item {
        let i = y.unchecked_mul(self.width).unchecked_add(x);
        self.as_slice().get_unchecked(i)
    }

    #[inline]
    fn row_slice(&self, y: usize) -> Option<&[Self::Item]> {
        y.checked_mul(self.width)
            .and_then(|i| self.as_slice().get(i..(i + self.width)))
    }
}

impl<T, S: AsRef<[T]> + AsMut<[T]>> GridMut for GridBuf<T, S> {
    type RootMut = Self;

    #[inline]
    fn root_mut(&mut self) -> &mut Self::RootMut {
        self
    }

    #[inline]
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item> {
        y.checked_mul(self.width)
            .and_then(|y| y.checked_add(x))
            .and_then(|i| self.as_mut_slice().get_mut(i))
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut Self::Item {
        let i = y.unchecked_mul(self.width).unchecked_add(x);
        self.as_mut_slice().get_unchecked_mut(i)
    }

    #[inline]
    fn row_slice_mut(&mut self, y: usize) -> Option<&mut [Self::Item]> {
        let w = self.width;
        y.checked_mul(w)
            .and_then(|i| self.as_mut_slice().get_mut(i..(i + w)))
    }
}

impl<'a, T, S: AsRef<[T]>> IntoIterator for &'a GridBuf<T, S> {
    type Item = (&'a T, usize, usize);
    type IntoIter = GridIter<&'a GridBuf<T, S>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, S: AsRef<[T]> + AsMut<[T]>> IntoIterator for &'a mut GridBuf<T, S> {
    type Item = (&'a mut T, usize, usize);
    type IntoIter = GridIter<&'a mut GridBuf<T, S>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, S: Clone> Clone for GridBuf<T, S> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            store: self.store.clone(),
            marker: PhantomData,
        }
    }
}

impl<T: Debug, S: AsRef<[T]>> Debug for GridBuf<T, S> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.debug_fmt(f)
    }
}

impl<C: Coord, T, S: AsRef<[T]>> Index<C> for GridBuf<T, S> {
    type Output = T;

    #[inline]
    fn index(&self, index: C) -> &Self::Output {
        let (w, h) = (self.width, self.height);
        self.get(
            index.x().to_grid(w).expect("invalid x-coordinate"),
            index.y().to_grid(h).expect("invalid y-coordinate"),
        )
        .expect("coordinate out of bounds")
    }
}

impl<C: Coord, T, S: AsRef<[T]> + AsMut<[T]>> IndexMut<C> for GridBuf<T, S> {
    #[inline]
    fn index_mut(&mut self, index: C) -> &mut Self::Output {
        let (w, h) = (self.width, self.height);
        self.get_mut(
            index.x().to_grid(w).expect("invalid x-coordinate"),
            index.y().to_grid(h).expect("invalid y-coordinate"),
        )
        .expect("coordinate out of bounds")
    }
}
