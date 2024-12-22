//! A crate to make working with 2D arrays (grids) very pleasant.
//!
//! This library provides two core traits, [`Grid`] and [`GridMut`], which represent immutable
//! and mutable grids respectively. Rather than just supplying a concrete 2D array type, this
//! approach allows all grid-based algorithms to be written generically, which lets the user
//! choose the actual implementation and storage method for their grids.
//!
//! In addition to these traits, a [`GridBuf`] is provided which allows you to create a grid
//! out of any collection that implements [`AsRef`] and optionally [`AsMut`] (such as arrays,
//! slices, vectors, and tiny/smallvec types).
//!
//! # Examples
//!
//! The simplest way to create a grid is to create a 2D array. The grid traits are implemented
//! on all arrays of the form `[[T; WIDTH]; HEIGHT]`.
//!
//! ```rust
//! use grid::{Grid, GridMut};
//!
//! // create a 3×2 grid of letters
//! let mut letters = [
//!     ['A', 'B', 'C'],
//!     ['D', 'E', 'F']
//! ];
//!
//! // you can extract values from it
//! assert_eq!(letters.get(1, 0), Some(&'B'));
//! assert_eq!(letters.get(2, 1), Some(&'F'));
//!
//! // you can write values to it
//! letters.set(2, 0, 'X');
//! assert_eq!(letters.get(2, 0), Some(&'X'));
//! ```

mod grid;
mod grid_buf;
mod grid_iter;
mod grid_iter_mut;
mod grid_mut;
mod row;
mod row_iter;
mod rows_iter;
mod view;

pub use grid::*;
pub use grid_buf::*;
pub use grid_iter::*;
pub use grid_iter_mut::*;
pub use grid_mut::*;
pub use row::*;
pub use row_iter::*;
pub use rows_iter::*;
pub use view::*;

#[test]
fn test() {
    use std::fmt::{Debug, Write};
    fn display<T: Debug, G: Grid<Item = T>>(grid: &G) {
        let mut s = String::new();
        let mut len = 0;
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let val = grid.get(x, y).unwrap();
                s.clear();
                write!(s, "{:?}", val).unwrap();
                len = len.max(s.len());
            }
        }
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let val = grid.get(x, y).unwrap();
                s.clear();
                write!(s, "{:?}", val).unwrap();
                while s.len() < len {
                    s.push(' ');
                }
                print!("[{}]", s);
            }
            println!();
        }
        println!();
    }

    /*
    let mut grid = GridBuf::with_store(5, 5, vec![0usize; 25]);

    for (y, mut row) in grid.rows_mut().enumerate() {
        for (i, val) in row.iter_mut().enumerate() {
            *val = y * 10 + i;
        }
    }

    /*for x in 0..grid.width() {
        for y in 0..grid.height() {
            *grid.get_mut(x, y).unwrap() = y * grid.width() + x;
        }
    }*/

    display(&grid);

    //grid.rows_mut().last().unwrap().fill(9);

    let mut targ = grid.clone();

    for (mut dst, src) in targ.rows_mut().zip(grid.rows().rev()) {
        for (mut dst, src) in dst.iter_mut().zip(src.iter()) {
            *dst = *src;
        }
    }

    display(&targ);*/
}
