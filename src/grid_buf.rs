use crate::{Grid, GridIter, GridMut};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

/// A grid implementation for different storage types.
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

    #[inline]
    pub fn to_store(self) -> S {
        self.store
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
    #[inline]
    fn root_mut(&mut self) -> &mut Self::Root {
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

impl<T, Store: Serialize> Serialize for GridBuf<T, Store> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("GridBuf", 3)?;
        ser.serialize_field("width", &self.width)?;
        ser.serialize_field("height", &self.height)?;
        ser.serialize_field("store", &self.store)?;
        ser.end()
    }
}

impl<'de, T, S: Deserialize<'de>> Deserialize<'de> for GridBuf<T, S> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename(deserialize = "GridBuf"))]
        struct GridBufDe<S> {
            width: usize,
            height: usize,
            store: S,
        }
        let GridBufDe {
            width,
            height,
            store,
        } = GridBufDe::deserialize(deserializer)?;
        Ok(Self {
            width,
            height,
            store,
            marker: PhantomData,
        })
    }
}

impl<T: Debug, S: AsRef<[T]>> Debug for GridBuf<T, S> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.debug_fmt(f)
    }
}
