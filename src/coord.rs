use num_traits::NumOps;
use std::num::{NonZero, NonZeroUsize};

#[derive(
    Debug,
    Copy,
    Clone,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

pub const fn coord<T>(x: T, y: T) -> Coord<T> {
    Coord { x, y }
}

impl<T> Coord<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn splat(val: T) -> Self
    where
        T: Copy,
    {
        coord(val, val)
    }

    #[inline]
    pub fn as_index(&self, width: NonZeroUsize) -> Option<usize> {
        /*self.y
        .checked_mul(width.get())
        .and_then(|i| i.checked_add(self.x))*/
    }

    /*#[inline]
    pub fn as_index(&self, width: NonZeroUsize) -> Option<usize> {
        self.y
        .checked_mul(width.get())
        .and_then(|i| i.checked_add(self.x))
    }

    #[inline]
    pub fn get_value<'a, T>(&self, slice: &'a [T], width: NonZeroUsize) -> Option<&'a T> {
        self.as_index(width).and_then(|i| slice.get(i))
    }*/

    #[inline]
    pub fn min(self, other: Self) -> Self {
        coord(self.x.min(other.x), self.y.min(other.y))
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        coord(self.x.max(other.x), self.y.max(other.y))
    }

    #[inline]
    pub fn in_bounds(self, size: Self) -> bool {
        self.x < size.x && self.y < size.y
    }
}

impl From<(usize, usize)> for Coord {
    #[inline]
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl From<Coord> for (usize, usize) {
    #[inline]
    fn from(Coord { x, y }: Coord) -> Self {
        (x, y)
    }
}

macro_rules! impl_op {
    ($op:tt, $op_fn:tt, $assign:tt, $assign_fn:tt) => {
        impl std::ops::$op<Coord> for Coord {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: Coord) -> Self::Output {
                coord(self.x.$op_fn(rhs.x), self.y.$op_fn(rhs.y))
            }
        }

        impl std::ops::$op<&Coord> for Coord {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: &Coord) -> Self::Output {
                coord(self.x.$op_fn(rhs.x), self.y.$op_fn(rhs.y))
            }
        }

        impl std::ops::$op<Coord> for &Coord {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: Coord) -> Self::Output {
                coord(self.x.$op_fn(rhs.x), self.y.$op_fn(rhs.y))
            }
        }

        impl std::ops::$op<&Coord> for &Coord {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: &Coord) -> Self::Output {
                coord(self.x.$op_fn(rhs.x), self.y.$op_fn(rhs.y))
            }
        }

        impl std::ops::$assign<Coord> for Coord {
            #[inline]
            fn $assign_fn(&mut self, rhs: Coord) {
                self.x.$assign_fn(rhs.x);
                self.y.$assign_fn(rhs.y);
            }
        }

        impl std::ops::$assign<&Coord> for Coord {
            #[inline]
            fn $assign_fn(&mut self, rhs: &Coord) {
                self.x.$assign_fn(rhs.x);
                self.y.$assign_fn(rhs.y);
            }
        }

        impl std::ops::$op<(usize, usize)> for Coord {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: (usize, usize)) -> Self::Output {
                coord(self.x.$op_fn(rhs.0), self.y.$op_fn(rhs.1))
            }
        }

        impl std::ops::$op<&(usize, usize)> for Coord {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: &(usize, usize)) -> Self::Output {
                coord(self.x.$op_fn(rhs.0), self.y.$op_fn(rhs.1))
            }
        }

        impl std::ops::$op<(usize, usize)> for &Coord {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: (usize, usize)) -> Self::Output {
                coord(self.x.$op_fn(rhs.0), self.y.$op_fn(rhs.1))
            }
        }

        impl std::ops::$op<&(usize, usize)> for &Coord {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: &(usize, usize)) -> Self::Output {
                coord(self.x.$op_fn(rhs.0), self.y.$op_fn(rhs.1))
            }
        }

        impl std::ops::$assign<(usize, usize)> for Coord {
            #[inline]
            fn $assign_fn(&mut self, rhs: (usize, usize)) {
                self.x.$assign_fn(rhs.0);
                self.y.$assign_fn(rhs.1);
            }
        }

        impl std::ops::$assign<&(usize, usize)> for Coord {
            #[inline]
            fn $assign_fn(&mut self, rhs: &(usize, usize)) {
                self.x.$assign_fn(rhs.0);
                self.y.$assign_fn(rhs.1);
            }
        }
    };
}

macro_rules! impl_op_with_scalar {
    ($op:tt, $op_fn:tt, $assign:tt, $assign_fn:tt) => {
        impl_op!($op, $op_fn, $assign, $assign_fn);

        impl std::ops::$op<usize> for Coord {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: usize) -> Self::Output {
                coord(self.x.$op_fn(rhs), self.y.$op_fn(rhs))
            }
        }

        impl std::ops::$op<usize> for &Coord {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: usize) -> Self::Output {
                coord(self.x.$op_fn(rhs), self.y.$op_fn(rhs))
            }
        }

        impl std::ops::$op<&usize> for Coord {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: &usize) -> Self::Output {
                coord(self.x.$op_fn(*rhs), self.y.$op_fn(*rhs))
            }
        }

        impl std::ops::$op<&usize> for &Coord {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: &usize) -> Self::Output {
                coord(self.x.$op_fn(*rhs), self.y.$op_fn(*rhs))
            }
        }

        impl std::ops::$op<Coord> for usize {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: Coord) -> Self::Output {
                coord(rhs.x.$op_fn(self), rhs.y.$op_fn(self))
            }
        }

        impl std::ops::$op<Coord> for &usize {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: Coord) -> Self::Output {
                coord(rhs.x.$op_fn(*self), rhs.y.$op_fn(*self))
            }
        }

        impl std::ops::$op<&Coord> for usize {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: &Coord) -> Self::Output {
                coord(rhs.x.$op_fn(self), rhs.y.$op_fn(self))
            }
        }

        impl std::ops::$op<&Coord> for &usize {
            type Output = Coord;

            #[inline]
            fn $op_fn(self, rhs: &Coord) -> Self::Output {
                coord(rhs.x.$op_fn(*self), rhs.y.$op_fn(*self))
            }
        }
    };
}

impl_op!(Add, add, AddAssign, add_assign);
impl_op!(Sub, sub, SubAssign, sub_assign);
impl_op_with_scalar!(Mul, mul, MulAssign, mul_assign);
impl_op_with_scalar!(Div, div, DivAssign, div_assign);
impl_op_with_scalar!(Rem, rem, RemAssign, rem_assign);
impl_op_with_scalar!(Shl, shl, ShlAssign, shl_assign);
impl_op_with_scalar!(Shr, shr, ShrAssign, shr_assign);
impl_op_with_scalar!(BitAnd, bitand, BitAndAssign, bitand_assign);
impl_op_with_scalar!(BitOr, bitor, BitOrAssign, bitor_assign);
impl_op_with_scalar!(BitXor, bitxor, BitXorAssign, bitxor_assign);
