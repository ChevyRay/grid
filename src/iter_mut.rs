use crate::GridMut;

pub struct IterMut<'a, G> {
    grid: &'a mut G,
    x: usize,
    y: usize,
}

impl<'a, G> IterMut<'a, G> {
    #[inline]
    pub fn new(grid: &'a mut G) -> Self {
        Self { grid, x: 0, y: 0 }
    }
}

impl<'a, G: GridMut> Iterator for IterMut<'a, G> {
    type Item = (&'a mut G::Item, usize, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let val: *mut G::Item = self.grid.get_mut(self.x, self.y)?;
        let x = self.x;
        let y = self.y;
        self.x += 1;
        if self.x == self.grid.width() {
            self.x = 0;
            self.y += 1;
        }
        // SAFETY: this iterator has a mutable reference to the grid, so as long as it exists, the
        // mutable reference to the value fetched from inside the grid will also be valid.
        Some((unsafe { &mut *val }, x, y))
    }
}
