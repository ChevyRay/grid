use crate::{Grid, Iter};

pub struct View<'a, G> {
    grid: &'a G,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl<'a, G> View<'a, G> {
    #[inline]
    pub(crate) fn new(grid: &'a G, x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { grid, x, y, w, h }
    }

    #[inline]
    pub fn x(&self) -> usize {
        self.x
    }

    #[inline]
    pub const fn y(&self) -> usize {
        self.y
    }
}

impl<G: Grid> Grid for View<'_, G> {
    type Item = G::Item;
    type Root = G;

    #[inline]
    fn root(&self) -> &Self::Root {
        self.grid
    }

    #[inline]
    fn width(&self) -> usize {
        self.w
    }

    #[inline]
    fn height(&self) -> usize {
        self.h
    }

    #[inline]
    fn get(&self, x: usize, y: usize) -> Option<&Self::Item> {
        if x < self.w && y < self.h {
            self.root().get(self.x + x, self.y + y)
        } else {
            None
        }
    }

    #[inline]
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Item {
        self.root().get_unchecked(self.x + x, self.y + y)
    }

    #[inline]
    fn row_slice(&self, y: usize) -> Option<&[Self::Item]> {
        if y < self.h {
            self.grid
                .row_slice(self.y + y)
                .and_then(|s| s.get(self.x..(self.x + self.w)))
        } else {
            None
        }
    }

    #[inline]
    fn try_view(&self, x: usize, y: usize, w: usize, h: usize) -> Option<View<'_, Self::Root>> {
        if x + w <= self.w && y + h <= self.h {
            Some(View::new(self.grid, self.x + x, self.y + y, w, h))
        } else {
            None
        }
    }
}

impl<'a, G: Grid> IntoIterator for &'a View<'a, G> {
    type Item = (&'a G::Item, usize, usize);
    type IntoIter = Iter<'a, View<'a, G>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
