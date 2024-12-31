use crate::Coord;
use glam::{I16Vec2, I64Vec2, I8Vec2, IVec2, U16Vec2, U64Vec2, U8Vec2, UVec2};

macro_rules! impl_coord {
    ($vec:ident $prim:ty) => {
        impl Coord for $vec {
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
    };
}

impl_coord!(U8Vec2 u8);
impl_coord!(I8Vec2 i8);
impl_coord!(U16Vec2 u16);
impl_coord!(I16Vec2 i16);
impl_coord!(UVec2 u32);
impl_coord!(IVec2 i32);
impl_coord!(U64Vec2 u64);
impl_coord!(I64Vec2 i64);
