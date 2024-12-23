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
//! You can think of having a [`View`] into a grid as the same as having a *slice*
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
//! // fill in the center 3×2 cells
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
//!
//! # GridBufs
//!
//! It is very common to store grid data in a single contiguous array or vector of data. If
//! you have a grid that is `width × height`, you can store its data in a list of that area
//! and access elements positionally with the formula `y * width + x`.
//!
//! The `GridBuf` struct is a wrapper over any type that implements `AsRef<[T]>` and/or
//! `AsMut<[T]>` that does all this for you and allows you to treat the data as if it
//! was stored in a grid. This has a few different useful purposes.
//!
//! You can create a heap-allocated grid:
//!
//! ```rust
//! use grid::{GridBuf, GridMut};
//!
//! let mut numbers: GridBuf<i32> = GridBuf::new(3, 3);
//! numbers.set(1, 1, 3);
//!
//! assert_eq!(numbers.into_store(), vec![
//!     0, 0, 0,
//!     0, 3, 0,
//!     0, 0, 0,
//! ]);
//! ```
//!
//! If you have a pre-existing vector, you could create one from that:
//!
//! ```rust
//! # use grid::{GridBuf, Grid};
//! let vec = vec![
//!     0, 1, 2,
//!     3, 4, 5,
//!     6, 7, 8
//! ];
//! let numbers = GridBuf::with_store(3, 3, vec);
//!
//! assert_eq!(numbers.get(0, 1), Some(&3));
//! assert_eq!(numbers.get(2, 2), Some(&8));
//! ```
//!
//! Maybe you don't want to heap-allocate, in which case you can just pass in a
//! stack-allocated array to use as the backing store:
//!
//! ```rust
//! # use grid::{GridBuf, GridMut};
//! let mut numbers = GridBuf::with_store(3, 3, [0i32; 9]);
//! numbers.set(0, 0, 1);
//! numbers.set(1, 1, 2);
//! numbers.set(2, 2, 3);
//!
//! assert_eq!(numbers.into_store(), [
//!     1, 0, 0,
//!     0, 2, 0,
//!     0, 0, 3,
//! ]);
//!```
//!
//! No heap-allocations required! You can store the data however you want, recycle it
//! between calls, or edit data in-place with a more convenient API. For example, if you
//! had a big lump of 2D data embedded in a program, you could just store it in a
//! contiguous static slice and wrap a `GridBuf` around it whenever you wanted to edit it.
//!
//! # Drawing
//!
//! In addition to positionally editing and sampling grids, you can also copy them from
//! each other. For example, if I have one grid, I can "draw" another grid on top of it
//! like so:
//!
//! ```rust
//! use grid::{GridBuf, Grid, GridMut};
//!
//! let mut dst = [
//!     [0, 0, 0, 0],
//!     [0, 0, 0, 0],
//!     [0, 0, 0, 0],
//!     [0, 0, 0, 0],
//! ];
//!
//! dst.view_mut(1, 1, 2, 2).copy_from(&[
//!     [1, 2],
//!     [3, 4],
//! ]);
//!
//! assert_eq!(dst, [
//!     [0, 0, 0, 0],
//!     [0, 1, 2, 0],
//!     [0, 3, 4, 0],
//!     [0, 0, 0, 0],
//! ]);
//! ```
//!
//! If two grids are the same size, one can be "pasted" onto the other. So to paint just
//! the middle 2×2 section, we get a view of it and then draw another 2×2 grid on top.
//!
//! # Generic Grids
//!
//! Because the entire API uses the the [`Grid`] and [`GridMut`] traits, it is very easy
//! to write algorithms that can work on any kinds of grids with any sort of data storage,
//! with almost no glue required to make them work together.
//!
//! When writing an algorithm to operate on grids, best practics is to do so generically.
//! For example, I could write an algorithm that draws an empty box:
//!
//! ```rust
//! use grid::{Grid, GridMut};
//!
//! fn draw_box<T, G>(grid: &mut G, value: T, x: usize, y: usize, w: usize, h: usize)
//! where
//!     T: Clone,
//!     G: GridMut<Item = T>,
//! {
//!     let mut view = grid.view_mut(x, y, w, h);
//!     view.row_mut(0).fill(value.clone());
//!     view.row_mut(h - 1).fill(value.clone());
//!     view.col_mut(0).fill(value.clone());
//!     view.col_mut(w - 1).fill(value);
//! }
//!
//! let mut nums = [
//!     [0, 0, 0, 0, 0, 0],
//!     [0, 0, 0, 0, 0, 0],
//!     [0, 0, 0, 0, 0, 0],
//!     [0, 0, 0, 0, 0, 0],
//!     [0, 0, 0, 0, 0, 0],
//!     [0, 0, 0, 0, 0, 0],
//! ];
//!
//! draw_box(&mut nums, 8, 1, 1, 4, 4);
//!
//! assert_eq!(nums, [
//!     [0, 0, 0, 0, 0, 0],
//!     [0, 8, 8, 8, 8, 0],
//!     [0, 8, 0, 0, 8, 0],
//!     [0, 8, 0, 0, 8, 0],
//!     [0, 8, 8, 8, 8, 0],
//!     [0, 0, 0, 0, 0, 0],
//! ]);
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
