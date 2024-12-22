use crate::Grid;
use std::iter::FusedIterator;

#[derive(Copy, Clone)]
pub struct GridIter<'a, G> {
    grid: &'a G,
    x: usize,
    y: usize,
}

impl<'a, G> GridIter<'a, G> {
    #[inline]
    pub(crate) fn new(grid: &'a G) -> Self {
        Self { grid, x: 0, y: 0 }
    }
}

impl<'a, G: Grid> Iterator for GridIter<'a, G> {
    type Item = (&'a G::Item, usize, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let val = self.grid.get(self.x, self.y)?;
        let x = self.x;
        let y = self.y;
        self.x += 1;
        if self.x == self.grid.width() {
            self.x = 0;
            self.y += 1;
        }
        Some((val, x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }
}

impl<G: Grid> ExactSizeIterator for GridIter<'_, G> {
    #[inline]
    fn len(&self) -> usize {
        let w = self.grid.width();
        let h = self.grid.height();
        (h.saturating_sub(self.y + 1)) * w + (w - self.x)
    }
}

impl<G: Grid> FusedIterator for GridIter<'_, G> {}
