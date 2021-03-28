use crate::{GridIndex, GridIter, GridIterMut, GridIterUnchecked, GridIterUncheckedMut};
use math::{int2, irect, Int2, IntRect};
use std::marker::PhantomData;

pub trait Grid<T> {
    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn get<I: GridIndex<T>>(&self, index: I) -> Option<&T>;

    fn get_mut<I: GridIndex<T>>(&mut self, index: I) -> Option<&mut T>;

    unsafe fn get_unchecked<I: GridIndex<T>>(&self, index: I) -> &T;

    unsafe fn get_unchecked_mut<I: GridIndex<T>>(&mut self, index: I) -> &mut T;

    fn set<I: GridIndex<T>, U: Into<T>>(&mut self, index: I, value: U) {
        if let Some(target) = self.get_mut(index) {
            *target = value.into();
        }
    }

    unsafe fn set_unchecked<I: GridIndex<T>, U: Into<T>>(&mut self, index: I, value: U) {
        *self.get_unchecked_mut(index) = value.into();
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

    unsafe fn fill_with_unchecked<U: Into<T>, F: FnMut(Int2) -> U>(
        &mut self,
        rect: IntRect,
        mut f: F,
    ) {
        if let Some(rect) = self.bounds().overlap(&rect) {
            for p in rect.iter() {
                self.set_unchecked(p, f(p).into())
            }
        }
    }

    fn fill<U: Clone + Into<T>>(&mut self, rect: IntRect, value: U) {
        self.fill_with(rect, |_| value.clone());
    }

    unsafe fn fill_unchecked<U: Clone + Into<T>>(&mut self, rect: IntRect, value: U) {
        self.fill_with_unchecked(rect, |_| value.clone());
    }

    fn clear_with<U: Into<T>, F: FnMut(Int2) -> U>(&mut self, f: F) {
        self.fill_with(self.bounds(), f);
    }

    unsafe fn clear_with_unchecked<U: Into<T>, F: FnMut(Int2) -> U>(&mut self, f: F) {
        self.fill_with_unchecked(self.bounds(), f);
    }

    fn clear<U: Clone + Into<T>>(&mut self, value: U) {
        self.fill_with(self.bounds(), |_| value.clone());
    }

    unsafe fn clear_unchecked<U: Clone + Into<T>>(&mut self, value: U) {
        self.fill_with_unchecked(self.bounds(), |_| value.clone());
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

    unsafe fn get_bounds_unchecked<C: FnMut(&T) -> bool>(&self, mut cond: C) -> Option<IntRect> {
        let mut min = int2(i32::MIN, i32::MIN);
        let mut max = int2(i32::MAX, i32::MAX);
        for p in self.bounds().iter() {
            let val = self.get_unchecked(p);
            if cond(val) {
                min = min.min(p);
                max = max.max(p);
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

    unsafe fn clone_from_unchecked<U: Clone + Into<T>, G: Grid<U>>(
        &mut self,
        grid: &G,
        rect: IntRect,
        dest: Int2,
    ) {
        if let Some(rect) = grid.bounds().overlap(&rect) {
            if let Some(rect) = self.bounds().overlap(&(rect + dest)) {
                for p in rect.iter() {
                    self.set_unchecked(dest + p, grid.get_unchecked(p).clone());
                }
            }
        }
    }

    fn in_rect(&self, rect: IntRect) -> GridIter<T, Self>
    where
        Self: Sized,
    {
        let rect = self
            .bounds()
            .overlap(&rect)
            .unwrap_or_else(|| IntRect::EMPTY);
        GridIter {
            grid: self,
            iter: rect.iter(),
            _marker: PhantomData::default(),
        }
    }

    fn in_rect_unchecked(&self, rect: IntRect) -> GridIterUnchecked<T, Self>
    where
        Self: Sized,
    {
        let rect = self
            .bounds()
            .overlap(&rect)
            .unwrap_or_else(|| IntRect::EMPTY);
        GridIterUnchecked {
            grid: self,
            iter: rect.iter(),
            _marker: PhantomData::default(),
        }
    }

    fn in_rect_mut(&mut self, rect: IntRect) -> GridIterMut<T, Self>
    where
        Self: Sized,
    {
        let rect = self
            .bounds()
            .overlap(&rect)
            .unwrap_or_else(|| IntRect::EMPTY);
        GridIterMut {
            grid: self,
            iter: rect.iter(),
            _marker: PhantomData::default(),
        }
    }

    unsafe fn in_rect_unchecked_mut(&mut self, rect: IntRect) -> GridIterUncheckedMut<T, Self>
    where
        Self: Sized,
    {
        let rect = self
            .bounds()
            .overlap(&rect)
            .unwrap_or_else(|| IntRect::EMPTY);
        GridIterUncheckedMut {
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

    unsafe fn iter_unchecked(&self) -> GridIterUnchecked<T, Self>
    where
        Self: Sized,
    {
        self.in_rect_unchecked(self.bounds())
    }

    fn iter_mut(&mut self) -> GridIterMut<T, Self>
    where
        Self: Sized,
    {
        self.in_rect_mut(self.bounds())
    }

    unsafe fn iter_unchecked_mut(&mut self) -> GridIterUncheckedMut<T, Self>
    where
        Self: Sized,
    {
        self.in_rect_unchecked_mut(self.bounds())
    }
}
