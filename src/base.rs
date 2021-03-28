use crate::GridIndex;
use math::{int2, irect, Int2, IntRect, IntRectIter};
use std::marker::PhantomData;

pub trait Grid<T> {
    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn get<I: GridIndex<T>>(&self, index: I) -> Option<&T>;

    fn get_mut<I: GridIndex<T>>(&mut self, index: I) -> Option<&mut T>;

    fn set<I: GridIndex<T>, U: Into<T>>(&mut self, index: I, value: U) {
        if let Some(target) = self.get_mut(index) {
            *target = value.into();
        }
    }

    fn bounds(&self) -> IntRect {
        irect(0, 0, self.width() as i32, self.height() as i32)
    }

    fn fill_with<U: Into<T>, F: FnMut(Int2) -> U>(&mut self, rect: IntRect, mut f: F) {
        if let Some(rect) = self.bounds().overlap(&rect) {
            for p in rect.iter() {
                self.set(p, f(p).into())
            }
        }
    }

    fn fill<U: Clone + Into<T>>(&mut self, rect: IntRect, value: U) {
        self.fill_with(rect, |_| value.clone());
    }

    fn clear_with<U: Into<T>, F: FnMut(Int2) -> U>(&mut self, f: F) {
        self.fill_with(self.bounds(), f);
    }

    fn clear<U: Clone + Into<T>>(&mut self, value: U) {
        self.fill_with(self.bounds(), |_| value.clone());
    }

    fn get_bounds<C: FnMut(&T) -> bool>(&self, mut cond: C) -> Option<IntRect> {
        let mut min = int2(i32::MIN, i32::MIN);
        let mut max = int2(i32::MAX, i32::MAX);
        for p in self.bounds().iter() {
            if let Some(val) = self.get(p) {
                if cond(val) {
                    min = min.min(p);
                    max = max.max(p);
                }
            }
        }
        (max.x > min.x && max.y > min.y).then(|| irect(min.x, min.y, max.x - min.x, max.y - min.y))
    }

    fn clone_from<U: Clone + Into<T>, G: Grid<U>>(&mut self, grid: &G, rect: IntRect, dest: Int2) {
        if let Some(rect) = grid.bounds().overlap(&rect) {
            if let Some(rect) = self.bounds().overlap(&(rect + dest)) {
                for p in rect.iter() {
                    if let Some(val) = grid.get(p) {
                        self.set(dest + p, val.clone());
                    }
                }
            }
        }
    }

    fn in_rect(&self, rect: IntRect) -> GridIter<T, Self>
    where
        Self: Sized,
    {
        GridIter {
            grid: self,
            iter: rect.iter(),
            _marker: PhantomData::default(),
        }
    }

    fn in_rect_mut(&mut self, rect: IntRect) -> GridIterMut<T, Self>
    where
        Self: Sized,
    {
        GridIterMut {
            grid: self,
            iter: rect.iter(),
            _marker: PhantomData::default(),
        }
    }

    fn iter(&self) -> GridIter<T, Self>
    where
        Self: Sized,
    {
        self.in_rect(self.bounds())
    }

    fn iter_mut(&mut self) -> GridIterMut<T, Self>
    where
        Self: Sized,
    {
        self.in_rect_mut(self.bounds())
    }
}

pub struct GridIter<'a, T, G: Grid<T>> {
    grid: &'a G,
    iter: IntRectIter,
    _marker: PhantomData<T>,
}

impl<'a, T: 'a, G: Grid<T>> Iterator for GridIter<'a, T, G> {
    type Item = (Int2, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(pos) = self.iter.next() {
            if let Some(val) = self.grid.get(pos) {
                return Some((pos, val));
            }
        }
        None
    }
}

pub struct GridIterMut<'a, T, G: Grid<T>> {
    grid: &'a mut G,
    iter: IntRectIter,
    _marker: PhantomData<T>,
}

impl<'a, T: 'a, G: Grid<T>> Iterator for GridIterMut<'a, T, G> {
    type Item = (Int2, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(pos) = self.iter.next() {
            if let Some(val) = self.grid.get_mut(pos) {
                let ptr: *mut T = val;
                return unsafe { Some((pos, &mut *ptr)) };
            }
        }
        None
    }
}
