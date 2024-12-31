use crate::Coord;
use mint::Point2;

macro_rules! impl_coord {
    ($($prim:ident)*) => {
        $(
        impl Coord for Point2<$prim> {
            type X = $prim;
            type Y = $prim;

            #[inline]
            fn x(&self) -> Self::X {
                self.x
            }

            #[inline]
            fn y(&self) -> Self::Y {
                self.y
            }
        }

        impl Coord for &Point2<$prim> {
            type X = $prim;
            type Y = $prim;

            #[inline]
            fn x(&self) -> Self::X {
                self.x
            }

            #[inline]
            fn y(&self) -> Self::Y {
                self.y
            }
        }
        )*
    };
}

impl_coord!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize);
