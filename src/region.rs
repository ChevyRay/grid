// use crate::{coord, Coord};
//
// #[derive(
//     Debug,
//     Clone,
//     Default,
//     Eq,
//     PartialEq,
//     Ord,
//     PartialOrd,
//     Hash,
//     serde::Serialize,
//     serde::Deserialize,
// )]
// pub struct Region {
//     pub x: usize,
//     pub y: usize,
//     pub w: usize,
//     pub h: usize,
// }
//
// #[inline]
// pub const fn region(x: usize, y: usize, w: usize, h: usize) -> Region {
//     Region { x, y, w, h }
// }
//
// impl Region {
//     #[inline]
//     pub const fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
//         Self { x, y, w, h }
//     }
//
//     #[inline]
//     pub const fn size(&self) -> Coord<T> {
//         coord(self.w, self.h)
//     }
//
//     #[inline]
//     pub const fn top_left(&self) -> Coord<T> {
//         coord(self.x, self.y)
//     }
//
//     #[inline]
//     pub const fn bottom_right(&self) -> Option<Coord<T>> {
//         Some(coord(
//             self.x.checked_add(self.w.checked_sub(1)?)?,
//             self.y.checked_add(self.h.checked_sub(1)?)?,
//         ))
//     }
//
//     #[inline]
//     pub const fn range(&self) -> Option<(Coord, Coord)> {
//         self.bottom_right().map(|br| (self.top_left(), br))
//     }
// }
