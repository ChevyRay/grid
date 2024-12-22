use crate::{Grid, GridMut, Row};

#[derive(Clone)]
pub struct RowIter<RowRef> {
    pub(crate) row: RowRef,
    pub(crate) x: usize,
    pub(crate) w: usize,
}

impl<'a, G: Grid> Iterator for RowIter<&'a Row<&'a G>> {
    type Item = &'a G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.w {
            let val = self.row.get(self.x).unwrap();
            self.x += 1;
            Some(val)
        } else {
            None
        }
    }
}

impl<'a, G: Grid> Iterator for RowIter<&'a Row<&'a mut G>> {
    type Item = &'a G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.w {
            let val = self.row.get(self.x).unwrap();
            self.x += 1;
            Some(val)
        } else {
            None
        }
    }
}

impl<'a, G: GridMut> Iterator for RowIter<&'a mut Row<&'a mut G>> {
    type Item = &'a mut G::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let val: *mut G::Item = self.row.get_mut(self.x)?;
        self.x += 1;
        Some(unsafe { &mut *val })
    }
}
