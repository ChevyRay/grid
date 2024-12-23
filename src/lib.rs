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
//! # Basic Usage
//!
//! The simplest way to create a grid is to create a 2D array. The grid traits are implemented
//! on all arrays of the form `[[T; WIDTH]; HEIGHT]`. For example:
//!
//! ```rust
//! use grid::{Grid, GridMut};
//!
//! // create a 3×2 grid of letters
//! let mut letters = [
//!     ['A', 'B', 'C'],
//!     ['D', 'E', 'F'],
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
//!
//! You can operate on specific columns and rows:
//!
//! ```rust
//! # use grid::{Grid, GridMut};
//! let mut letters = [
//!     [' ', ' ', ' '],
//!     [' ', ' ', ' '],
//!     [' ', ' ', ' '],
//! ];
//!
//! letters.row_mut(1).fill('X');
//! assert_eq!(letters, [
//!     [' ', ' ', ' '],
//!     ['X', 'X', 'X'],
//!     [' ', ' ', ' '],
//! ]);
//!
//! letters.col_mut(2).fill('Y');
//! assert_eq!(letters, [
//!     [' ', ' ', 'Y'],
//!     ['X', 'X', 'Y'],
//!     [' ', ' ', 'Y'],
//! ]);
//! ```
//!
//! You can iterate over columns or rows:
//!
//! ```rust
//! # use grid::{Grid, GridMut};
//! let mut numbers = [
//!     [1, 2, 3],
//!     [4, 5, 6],
//!     [7, 8, 9],
//! ];
//!
//! let row_sums: Vec<i32> = numbers
//!     .rows()
//!     .map(|row| row.into_iter().sum())
//!     .collect();
//!
//! assert_eq!(row_sums, vec![6, 15, 24]);
//! ```
//!
//! Or operate on the entire grid at once:
//!
//! ```rust
//! # use grid::{Grid, GridMut};
//! let mut numbers = [
//!     [0, 0, 0],
//!     [0, 0, 0],
//!     [0, 0, 0],
//! ];
//!
//! numbers.fill(3);
//! assert_eq!(numbers, [
//!     [3, 3, 3],
//!     [3, 3, 3],
//!     [3, 3, 3],
//! ]);
//!
//! let mut digits = 0..;
//! numbers.fill_with(|| digits.next().unwrap());
//! assert_eq!(numbers, [
//!     [0, 1, 2],
//!     [3, 4, 5],
//!     [6, 7, 8],
//! ]);
//! ```
//!
//! # Views
//!
//! In addition to working on the grids themselves, you can create *views* into grids.
//! You can think of having a [`View`](View) into a grid as the same as having a *slice*
//! into a vector or array, except it is two dimensions instead of one.
//!
//! These views are themselves grids, allowing you to treat sub-sections of your root grid
//! as if they were entire grids themselves. This means that any algorithm that works with
//! grid traits will also work on sub-sections of a grid as well.
//!
//! For example, rather than filling an entire grid, I could fill in just a portion of it:
//!
//! ```rust
//! # use grid::{Grid, GridMut};
//! let mut block = [
//!     [0, 0, 0, 0, 0],
//!     [0, 0, 0, 0, 0],
//!     [0, 0, 0, 0, 0],
//!     [0, 0, 0, 0, 0],
//! ];
//!
//! // fill in the center 2×2 cells
//! block.view_mut(1, 1, 3, 2).fill(1);
//!
//! assert_eq!(block, [
//!     [0, 0, 0, 0, 0],
//!     [0, 1, 1, 1, 0],
//!     [0, 1, 1, 1, 0],
//!     [0, 0, 0, 0, 0],
//! ]);
//! ```
//!
//! When you have a view, all coordinates are local to its top-left:
//!
//! ```rust
//! # use grid::{Grid, GridMut};
//! let mut numbers = [
//!     ['A', 'B', 'C', 'D'],
//!     ['E', 'F', 'G', 'H'],
//!     ['I', 'J', 'K', 'L'],
//!     ['M', 'N', 'O', 'P'],
//! ];
//!
//! // (0, 0) is the top-left of the entire grid
//! assert_eq!(numbers.get(0, 0), Some(&'A'));
//!
//! // but if we have a "view" into just the middle 2×2 cells,
//! // then now (0, 0) is the top-left of that sub-section
//! let middle = numbers.view(1, 1, 2, 2);
//! assert_eq!(middle.get(0, 0), Some(&'F'));
//! ```

mod col;
mod col_iter;
mod cols_iter;
mod grid;
mod grid_buf;
mod grid_iter;
mod grid_iter_mut;
mod grid_mut;
mod row;
mod row_iter;
mod rows_iter;
mod view;

pub use col::*;
pub use col_iter::*;
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
