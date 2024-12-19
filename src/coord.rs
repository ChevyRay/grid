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
    pub const fn with_x(self, x: T) -> Self
    where
        T: Copy,
    {
        coord(x, self.y)
    }

    #[inline]
    pub const fn with_y(self, y: T) -> Self
    where
        T: Copy,
    {
        coord(self.x, y)
    }
}

impl<T> From<(T, T)> for Coord<T> {
    #[inline]
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> From<Coord<T>> for (T, T) {
    #[inline]
    fn from(Coord { x, y }: Coord<T>) -> Self {
        (x, y)
    }
}

macro_rules! impl_op {
    ($op:tt, $op_fn:tt, $assign:tt, $assign_fn:tt) => {
        impl<T: std::ops::$op<T, Output = T>> std::ops::$op<Coord<T>> for Coord<T> {
            type Output = Coord<T>;

            #[inline]
            fn $op_fn(self, rhs: Coord<T>) -> Self::Output {
                coord(self.x.$op_fn(rhs.x), self.y.$op_fn(rhs.y))
            }
        }

        impl<T: Copy + std::ops::$op<T, Output = T>> std::ops::$op<&Coord<T>> for Coord<T> {
            type Output = Coord<T>;

            #[inline]
            fn $op_fn(self, rhs: &Coord<T>) -> Self::Output {
                coord(self.x.$op_fn(rhs.x), self.y.$op_fn(rhs.y))
            }
        }

        impl<T: Copy + std::ops::$op<T, Output = T>> std::ops::$op<Coord<T>> for &Coord<T> {
            type Output = Coord<T>;

            #[inline]
            fn $op_fn(self, rhs: Coord<T>) -> Self::Output {
                coord(self.x.$op_fn(rhs.x), self.y.$op_fn(rhs.y))
            }
        }

        impl<T: Copy + std::ops::$op<T, Output = T>> std::ops::$op<&Coord<T>> for &Coord<T> {
            type Output = Coord<T>;

            #[inline]
            fn $op_fn(self, rhs: &Coord<T>) -> Self::Output {
                coord(self.x.$op_fn(rhs.x), self.y.$op_fn(rhs.y))
            }
        }

        impl<T: std::ops::$assign<T>> std::ops::$assign<Coord<T>> for Coord<T> {
            #[inline]
            fn $assign_fn(&mut self, rhs: Coord<T>) {
                self.x.$assign_fn(rhs.x);
                self.y.$assign_fn(rhs.y);
            }
        }

        impl<T: Copy + std::ops::$assign<T>> std::ops::$assign<&Coord<T>> for Coord<T> {
            #[inline]
            fn $assign_fn(&mut self, rhs: &Coord<T>) {
                self.x.$assign_fn(rhs.x);
                self.y.$assign_fn(rhs.y);
            }
        }

        impl<T: std::ops::$op<T, Output = T>> std::ops::$op<(T, T)> for Coord<T> {
            type Output = Coord<T>;

            #[inline]
            fn $op_fn(self, rhs: (T, T)) -> Self::Output {
                coord(self.x.$op_fn(rhs.0), self.y.$op_fn(rhs.1))
            }
        }

        impl<T: Copy + std::ops::$op<T, Output = T>> std::ops::$op<&(T, T)> for Coord<T> {
            type Output = Coord<T>;

            #[inline]
            fn $op_fn(self, rhs: &(T, T)) -> Self::Output {
                coord(self.x.$op_fn(rhs.0), self.y.$op_fn(rhs.1))
            }
        }

        impl<T: Copy + std::ops::$op<T, Output = T>> std::ops::$op<(T, T)> for &Coord<T> {
            type Output = Coord<T>;

            #[inline]
            fn $op_fn(self, rhs: (T, T)) -> Self::Output {
                coord(self.x.$op_fn(rhs.0), self.y.$op_fn(rhs.1))
            }
        }

        impl<T: Copy + std::ops::$op<T, Output = T>> std::ops::$op<&(T, T)> for &Coord<T> {
            type Output = Coord<T>;

            #[inline]
            fn $op_fn(self, rhs: &(T, T)) -> Self::Output {
                coord(self.x.$op_fn(rhs.0), self.y.$op_fn(rhs.1))
            }
        }

        impl<T: std::ops::$assign<T>> std::ops::$assign<(T, T)> for Coord<T> {
            #[inline]
            fn $assign_fn(&mut self, rhs: (T, T)) {
                self.x.$assign_fn(rhs.0);
                self.y.$assign_fn(rhs.1);
            }
        }

        impl<T: Copy + std::ops::$assign<T>> std::ops::$assign<&(T, T)> for Coord<T> {
            #[inline]
            fn $assign_fn(&mut self, rhs: &(T, T)) {
                self.x.$assign_fn(rhs.0);
                self.y.$assign_fn(rhs.1);
            }
        }
    };
}

macro_rules! impl_op_with_scalar {
    ($op:tt, $op_fn:tt, $assign:tt, $assign_fn:tt) => {
        impl_op!($op, $op_fn, $assign, $assign_fn);

        impl<T: Copy + std::ops::$op<T, Output = T>> std::ops::$op<T> for Coord<T> {
            type Output = Coord<T>;

            #[inline]
            fn $op_fn(self, rhs: T) -> Self::Output {
                coord(self.x.$op_fn(rhs), self.y.$op_fn(rhs))
            }
        }

        impl<T: Copy + std::ops::$op<T, Output = T>> std::ops::$op<T> for &Coord<T> {
            type Output = Coord<T>;

            #[inline]
            fn $op_fn(self, rhs: T) -> Self::Output {
                coord(self.x.$op_fn(rhs), self.y.$op_fn(rhs))
            }
        }

        impl<T: Copy + std::ops::$op<T, Output = T>> std::ops::$op<&T> for Coord<T> {
            type Output = Coord<T>;

            #[inline]
            fn $op_fn(self, rhs: &T) -> Self::Output {
                coord(self.x.$op_fn(*rhs), self.y.$op_fn(*rhs))
            }
        }

        impl<T: Copy + std::ops::$op<T, Output = T>> std::ops::$op<&T> for &Coord<T> {
            type Output = Coord<T>;

            #[inline]
            fn $op_fn(self, rhs: &T) -> Self::Output {
                coord(self.x.$op_fn(*rhs), self.y.$op_fn(*rhs))
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
