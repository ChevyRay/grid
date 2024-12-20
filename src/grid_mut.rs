use crate::{Grid, IterMut, ViewMut};

pub trait GridMut: Grid {
    fn root_mut(&mut self) -> &mut Self::Root;
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Self::Item>;

    /// # Safety
    ///
    /// Calling this method with an out-of-bounds coord is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut Self::Item;

    fn row_slice_mut(&mut self, y: usize) -> Option<&mut [Self::Item]>;

    #[inline]
    fn set(&mut self, x: usize, y: usize, value: Self::Item) -> Option<Self::Item> {
        self.get_mut(x, y)
            .map(|curr| std::mem::replace(curr, value))
    }

    /// # Safety
    ///
    /// Calling this method with an out-of-bounds coord is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[inline]
    unsafe fn set_unchecked(&mut self, x: usize, y: usize, value: Self::Item) -> Self::Item {
        std::mem::replace(self.get_unchecked_mut(x, y), value)
    }

    #[inline]
    fn try_view_mut(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) -> Option<ViewMut<'_, Self::Root>>
    where
        Self::Root: GridMut<Item = Self::Item>,
    {
        if x + w <= self.width() && y + h <= self.height() {
            let x = self.root_x() + x;
            let y = self.root_y() + y;
            Some(ViewMut::new(self.root_mut(), x, y, w, h))
        } else {
            None
        }
    }

    #[inline]
    fn view_mut(&mut self, x: usize, y: usize, w: usize, h: usize) -> ViewMut<'_, Self::Root>
    where
        Self::Root: GridMut<Item = Self::Item>,
    {
        self.try_view_mut(x, y, w, h)
            .expect("view does not overlap grid's bounds")
    }

    #[inline]
    fn full_view_mut(&mut self) -> ViewMut<'_, Self::Root>
    where
        Self::Root: GridMut<Item = Self::Item>,
    {
        let w = self.width();
        let h = self.height();
        ViewMut::new(self.root_mut(), 0, 0, w, h)
    }

    #[inline]
    fn iter_mut(&mut self) -> IterMut<'_, Self>
    where
        Self: Sized,
    {
        IterMut::new(self)
    }
}
