mod const_grid;
mod grid_index;
mod grid_t;
mod iterators;
mod sparse_grid;
mod vec_grid;

pub use const_grid::ConstGrid;
pub use grid_index::GridIndex;
pub use grid_t::Grid;
pub use iterators::{GridIter, GridIterMut, GridIterUnchecked, GridIterUncheckedMut};
pub use sparse_grid::SparseGrid;
pub use vec_grid::VecGrid;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let mut map = SparseGrid::<char>::default();
        map.set((9, 9), 'D');
        map.set((0, 0), 'A');
        map.set((2, 0), 'B');
        map.set((5, 5), 'E');
        map.set((1, 1), 'C');
        //println!("{}", map);
        for (pos, val) in map.iter() {
            println!("{:?} = {}", pos, val);
        }
    }
}
