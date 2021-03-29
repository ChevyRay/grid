use math::Int2;

/// Trait for a value that can be used to index into a [`Grid`](crate::Grid).
///
/// Implementations provided for [`Int2`](crate::math::Int2), `usize`, `(i32, i32)`, and `(usize, usize)`.
pub trait GridIndex<T> {
    fn index(&self, width: usize) -> usize;
    fn pos(&self, width: usize) -> (usize, usize);
}

impl<T> GridIndex<T> for usize {
    fn index(&self, _width: usize) -> usize {
        *self
    }
    fn pos(&self, width: usize) -> (usize, usize) {
        let index = *self;
        (width > 0)
            .then(|| (index % width, index / width))
            .unwrap_or((0, 0))
    }
}

impl<T> GridIndex<T> for (i32, i32) {
    fn index(&self, width: usize) -> usize {
        (self.1 * (width as i32) + self.0) as usize
    }

    fn pos(&self, _width: usize) -> (usize, usize) {
        (self.0 as usize, self.1 as usize)
    }
}

impl<T> GridIndex<T> for (usize, usize) {
    fn index(&self, width: usize) -> usize {
        self.1 * width + self.0
    }

    fn pos(&self, _width: usize) -> (usize, usize) {
        *self
    }
}

impl<T> GridIndex<T> for Int2 {
    fn index(&self, width: usize) -> usize {
        (self.y * (width as i32) + self.x) as usize
    }

    fn pos(&self, _width: usize) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}
