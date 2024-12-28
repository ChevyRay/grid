use std::fmt::Debug;
use std::hash::Hash;

/// A type that can be used as a grid coordinate.
pub trait Coord {
    /// The coordinate's scalar type.
    type Scalar: CoordScalar;

    /// The x-coordinate.
    fn x(&self) -> Self::Scalar;

    /// The y-coordinate.
    fn y(&self) -> Self::Scalar;

    /// Convert the x-coordinate to grid space, given the provided grid width.
    #[inline]
    fn prepare_x(&self, w: usize) -> Option<usize> {
        self.x().bounded(w)
    }

    /// Convert the y-coordinate to grid space, given the provided grid height.
    #[inline]
    fn prepare_y(&self, h: usize) -> Option<usize> {
        self.y().bounded(h)
    }
}

macro_rules! impl_coords {
    ($($type:ty)*) => {
        $(
        impl Coord for ($type, $type) {
            type Scalar = $type;

            #[inline]
            fn x(&self) -> Self::Scalar {
                self.0
            }

            #[inline]
            fn y(&self) -> Self::Scalar {
                self.1
            }
        }
        )*
    };
}

impl_coords!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize);

/// Wraps the inner coordinate in grid space.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Wrap<C>(pub C);

impl<C: Coord> Coord for Wrap<C> {
    type Scalar = C::Scalar;

    #[inline]
    fn x(&self) -> Self::Scalar {
        self.0.x()
    }

    #[inline]
    fn y(&self) -> Self::Scalar {
        self.0.y()
    }

    #[inline]
    fn prepare_x(&self, w: usize) -> Option<usize> {
        self.0.x().wrapped(w)
    }

    #[inline]
    fn prepare_y(&self, h: usize) -> Option<usize> {
        self.0.y().wrapped(h)
    }
}

/// Wraps the x value of the inner coordinate in grid space.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WrapX<C>(pub C);

impl<C: Coord> Coord for WrapX<C> {
    type Scalar = C::Scalar;

    #[inline]
    fn x(&self) -> Self::Scalar {
        self.0.x()
    }

    #[inline]
    fn y(&self) -> Self::Scalar {
        self.0.y()
    }

    #[inline]
    fn prepare_x(&self, w: usize) -> Option<usize> {
        self.0.x().wrapped(w)
    }
}

/// Wraps the y value of the inner coordinate in grid space.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WrapY<C>(pub C);

impl<C: Coord> Coord for WrapY<C> {
    type Scalar = C::Scalar;

    #[inline]
    fn x(&self) -> Self::Scalar {
        self.0.x()
    }

    #[inline]
    fn y(&self) -> Self::Scalar {
        self.0.y()
    }

    #[inline]
    fn prepare_y(&self, h: usize) -> Option<usize> {
        self.0.y().wrapped(h)
    }
}

/// Clamps the inner coordinate in grid space.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Clamp<C>(pub C);

impl<C: Coord> Coord for Clamp<C> {
    type Scalar = C::Scalar;

    #[inline]
    fn x(&self) -> Self::Scalar {
        self.0.x()
    }

    #[inline]
    fn y(&self) -> Self::Scalar {
        self.0.y()
    }

    #[inline]
    fn prepare_x(&self, w: usize) -> Option<usize> {
        self.0.x().clamped(w)
    }

    #[inline]
    fn prepare_y(&self, h: usize) -> Option<usize> {
        self.0.y().clamped(h)
    }
}

/// Clamps the x value of the inner coordinate in grid space.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ClampX<C>(pub C);

impl<C: Coord> Coord for ClampX<C> {
    type Scalar = C::Scalar;

    #[inline]
    fn x(&self) -> Self::Scalar {
        self.0.x()
    }

    #[inline]
    fn y(&self) -> Self::Scalar {
        self.0.y()
    }

    #[inline]
    fn prepare_x(&self, w: usize) -> Option<usize> {
        self.0.x().clamped(w)
    }
}

/// Clamps the y value of the inner coordinate in grid space.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ClampY<C>(pub C);

impl<C: Coord> Coord for ClampY<C> {
    type Scalar = C::Scalar;

    #[inline]
    fn x(&self) -> Self::Scalar {
        self.0.x()
    }

    #[inline]
    fn y(&self) -> Self::Scalar {
        self.0.y()
    }

    #[inline]
    fn prepare_y(&self, h: usize) -> Option<usize> {
        self.0.y().clamped(h)
    }
}

/// A value that can be used as an x or y coordinate.
pub trait CoordScalar: Debug + Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Hash {
    fn bounded(self, len: usize) -> Option<usize>;
    fn wrapped(self, len: usize) -> Option<usize>;
    fn clamped(self, len: usize) -> Option<usize>;
}

impl CoordScalar for usize {
    #[inline]
    fn bounded(self, len: usize) -> Option<usize> {
        (self < len).then_some(self)
    }

    #[inline]
    fn wrapped(self, len: usize) -> Option<usize> {
        self.checked_rem(len)
    }

    #[inline]
    fn clamped(self, len: usize) -> Option<usize> {
        len.checked_sub(1).map(|max| self.min(max))
    }
}

macro_rules! impl_unsigned {
    ($($type:ty)*) => {
        $(impl CoordScalar for $type {
            #[inline]
            fn bounded(self, len: usize) -> Option<usize> {
                (self as usize).bounded(len)
            }

            #[inline]
            fn wrapped(self, len: usize) -> Option<usize> {
                (self as usize).wrapped(len)
            }

            #[inline]
            fn clamped(self, len: usize) -> Option<usize> {
                (self as usize).clamped(len)
            }
        })*
    }
}

macro_rules! impl_signed {
    ($($type:ty)*) => {
        $(impl CoordScalar for $type {
            #[inline]
            fn bounded(self, len: usize) -> Option<usize> {
                if self < 0 {
                    return None;
                }
                (self as usize).bounded(len)
            }

            #[inline]
            fn wrapped(self, len: usize) -> Option<usize> {
                (self as usize).wrapping_add(len).checked_rem(len)
            }

            #[inline]
            fn clamped(self, len: usize) -> Option<usize> {
                if self < 0 {
                    return Some(0);
                }
                (self as usize).clamped(len)
            }
        })*
    }
}

impl_unsigned!(u8 u16 u32 u64);
impl_signed!(i8 i16 i32 i64 isize);
