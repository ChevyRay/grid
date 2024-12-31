//! # **▚▚▚ G R I D ▚▚▚**
//!
//! This library provides two core traits, [`Grid`] and [`GridMut`], which represent immutable
//! and mutable grids respectively. Rather than just supplying a concrete 2D array type, this
//! approach allows all grid-based algorithms to be written generically, which lets the user
//! choose the actual implementation and storage method for their grids.
//!
//! - [Use Cases](#use-cases)
//! - [Basic Usage](#basic-usage)
//! - [GridBufs](#gridbufs)
//! - [Views](#views)
//! - [Drawing](#drawing)
//! - [Coordinates](#coordinates)
//! - [Generic Code](#generic-code)
//! - [Roadmap](#roadmap)
//!
//! <div class="warning"><em>
//! This library is still an early draft, is missing features, and should be
//! considered volatile and subject to change. Once I think it has reached a
//! solid `Version 1.0` status, I will stabilize it and start applying proper
//! semantic versioning to changes. I am open-sourcing it early so I can get
//! feedback on the API, some of the implementations, and make improvements
//! before people start using it for actual projects.
//! </em></div>
//!
//! # Use Cases
//!
//! Being a game developer means that games, graphics, pathfinding, and procedural
//! generation are a huge part of what I do. This system for working with grids was mostly
//! inspired by my work in those areas, and is designed to serve them well.
//!
//! You can store entities in a grid, the pixels of an image, the walls of generated
//! mazes, a game map, or minesweeper mines. Whatever the case, having a very flexible and
//! modular API for working with these grids and making them interact with each other is
//! essential to having a good time.
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
//! use grid::{Grid, GridMut};
//!
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
//! use grid::{Grid, GridMut};
//!
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
//! use grid::{Grid, GridMut};
//!
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
//! # GridBufs
//!
//! It is very common to store grid data in a single contiguous array or vector of data. If
//! you have a grid that is `width × height`, you can store its data in a list of that area
//! and access elements positionally with the formula `y * width + x`.
//!
//! The [`GridBuf`] struct is a wrapper over any type that implements `AsRef<[T]>` and/or
//! `AsMut<[T]>` that does all this for you and allows you to treat the data as if it
//! was stored in a grid. This has a few different useful purposes. Three common forms
//! are provided: [`VecGrid`], [`ArrGrid`], and [`SliceGrid`].
//!
//! You can create a heap-allocated grid:
//!
//! ```rust
//! use grid::{VecGrid, GridMut};
//!
//! let mut numbers = VecGrid::new(3, 3);
//! numbers.set(1, 1, 3);
//!
//! assert_eq!(numbers.to_store(), vec![
//!     0, 0, 0,
//!     0, 3, 0,
//!     0, 0, 0,
//! ]);
//! ```
//!
//! If you have a pre-existing vector, you could create one from that:
//!
//! ```rust
//! use grid::{GridBuf, Grid};
//!
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
//! use grid::{GridBuf, GridMut};
//!
//! let mut numbers = GridBuf::with_store(3, 3, [0i32; 9]);
//! numbers.set(0, 0, 1);
//! numbers.set(1, 1, 2);
//! numbers.set(2, 2, 3);
//!
//! assert_eq!(numbers.to_store(), [
//!     1, 0, 0,
//!     0, 2, 0,
//!     0, 0, 3,
//! ]);
//! ```
//!
//! No heap-allocations required! You can store the data however you want, recycle it
//! between calls, or edit data in-place with a more convenient API. For example, if you
//! had a big lump of 2D data embedded in a program, you could just store it in a
//! contiguous static slice and wrap a `GridBuf` around it whenever you wanted to edit it.
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
//! use grid::{Grid, GridMut};
//!
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
//! use grid::{Grid, GridMut};
//!
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
//! dst.view_mut(1, 1, 2, 2).draw_copied(&[
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
//! # Coordinates
//!
//! In addition to the [`get`] and [`get_mut`] methods, there are also [`get_at`] and [`get_mut_at`]
//! alternatives that allow you to pass in any value that implements the [`Coord`] trait. This trait
//! is implemented on tuple pairs of all integer types, even signed values:
//!
//! ```
//! # use grid::Grid;
//! let mut nums = [
//!     [1, 2, 3],
//!     [4, 5, 6],
//!     [7, 8, 9],
//! ];
//!
//! assert_eq!(nums.get_at((1, 2)), Some(&8));
//! assert_eq!(nums.get_at((2i16, 1i16)), Some(&6));
//! ```
//!
//! It is very common, especially in games and graphical software, to use structs for points and
//! vectors that overload operators such as addition and multiplication, to make vector math a
//! lot more pleasant to write. In such a case, it is also very convenient to be able to use those
//! types as 2D coordinates in a grid. The [`Coord`] trait allows you to do just this:
//!
//! ```
//! use grid::{Grid, Coord};
//!
//! struct Point {
//!     x: i32,
//!     y: i32,
//! }
//!
//! impl Coord for Point {
//!     type X = i32;
//!     type Y = i32;
//!
//!     fn x(&self) -> Self::X {
//!         self.x
//!     }
//!
//!     fn y(&self) -> Self::Y {
//!         self.y
//!     }
//! }
//!
//! let mut nums = [
//!     [1, 2, 3],
//!     [4, 5, 6],
//!     [7, 8, 9],
//! ];
//!
//! assert_eq!(nums.get_at(Point { x: 1, y: 2 }), Some(&8));
//! assert_eq!(nums.get_at(Point { x: 2, y: 1 }), Some(&6));
//! ```
//!
//! Implementations are provided for many of the common math libraries behind features, such as
//! `cgmath`, `euclid`, `glam`, `mint`, and `vek`. So for example, if you are using `glam`, you
//! can enable that feature and use its integer vectors as grid coordinates:
//!
//! ```
//! # mod glam {
//! #     pub struct IVec2 { x: i32, y: i32 }
//! #     impl IVec2 { pub fn new(x: i32, y: i32) -> Self { Self { x, y } } }
//! #     impl grid::Coord for IVec2 {
//! #         type X = i32;
//! #         type Y = i32;
//! #         fn x(&self) -> i32 { self.x }
//! #         fn y(&self) -> i32 { self.y }
//! #     }
//! # }
//!
//! use grid::Grid;
//! use glam::IVec2;
//!
//! let mut nums = [
//!     [1, 2, 3],
//!     [4, 5, 6],
//!     [7, 8, 9],
//! ];
//!
//! assert_eq!(nums.get_at(IVec2::new(1, 2)), Some(&8));
//! assert_eq!(nums.get_at(IVec2::new(2, 1)), Some(&6));
//! ```
//!
//! [`get`]: Grid::get
//! [`get_mut`]: GridMut::get_mut
//! [`get_at`]: Grid::get_at
//! [`get_mut_at`]: GridMut::get_mut_at
//!
//! # Generic Code
//!
//! Because the entire API uses the the [`Grid`] and [`GridMut`] traits,
//! it is possible to write algorithms that can work on any kinds of grids with any sort of data storage,
//! with almost no glue required to make them work together.
//!
//! When writing an algorithm to operate on grids, best practics is to do so generically.
//! For example, I could write an algorithm that draws an empty box:
//!
//! ```rust
//! use grid::{Grid, GridMut};
//!
//! fn draw_box<T, G>(
//!     grid: &mut G,
//!     value: T,
//!     x: usize,
//!     y: usize,
//!     w: usize,
//!     h: usize,
//! )
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
//!
//! Because the function is written generically, it can be called on any type of grid,
//! with any sort of fill value. It could be numbers, chars, enums, structs, or anything.
//!
//! # Roadmap
//!
//! There are currently some missing features, traits that should be implemented, and
//! probably a few bugs to sniff out. I don't have an exact roadmap yet, but for now
//! I'll put a few notes here about what needs to be done.
//!
//! - [ ] It doesn't really have a name. I was thinking of calling it `moz` maybe? Unsure,
//!   open to suggestions on this. I don't know if I want to put this on [crates.io](crates.io),
//!   but if I was going to it would need a unique name.
//! - [ ] The crate's approach to iterators needs to be evaluated, as I feel like some
//!   iterators can be improved, some better naming conventions can be used, and there are
//!   probably certain useful iterators that are straight up missing.
//! - [ ] There are currently no tests, so one big task I need to do is write a full set
//!   of tests for all the constructors, methods, iterators, and trait implementations to
//!   make sure everything works correctly and to prevent future breaking changes.
//! - [ ] In addition to tests, I would like to fill out the documentation more, with
//!   more explanations of methods, `# Example` sections to show how to use them (and act as
//!   doc tests), and more clear documentation of which functions can panic and under which
//!   circumstances.
//! - [ ] Many essential std traits are likely not implemented yet that should be, and
//!   there are also probably traits that the library should be supplying that I have not
//!   discovered yet.
//! - [ ] Maybe `GridBuf`'s backing store should maybe be driven by a custom trait, or
//!   wrapper type, to make it more flexible. Not sure yet.
//! - [ ] I may want more grid implementations, such as a `HashMap`-backed grid, a
//!   sparse grid, or other useful features like that.
//! - [ ] Serde support should probably be feature-gated.
//! - [ ] I haven't written no-std stuff before, but I feel like a no-std version of
//!   this library could be possible, so I'd need to do some looking into that.
//! - [ ] Does some concept of a type representing a coordinate or rectangle fit into the
//!   scope of this library? The usefulness of coordinate types is already proven, but the
//!   current `usize` requirement of sampling grids poses some problems that need addressed:
//!   are all unsigned integers allowed? Are signed integers allowed? Is `-1` outside the
//!   bounds of a grid, or does it wrap around? Rust's standard library does not answer
//!   these questions and only uses `usize` for sampling, so there's no basis for this
//!   answer at least within the core libraries.

mod col;
mod col_iter;
mod cols_iter;
mod coord;
#[cfg(feature = "cgmath")]
mod feature_cgmath;
#[cfg(feature = "euclid")]
mod feature_euclid;
#[cfg(feature = "glam")]
mod feature_glam;
#[cfg(feature = "mint")]
mod feature_mint;
#[cfg(feature = "serde")]
mod feature_serde;
#[cfg(feature = "vek")]
mod features_vek;
mod grid;
mod grid_buf;
mod grid_iter;
mod grid_mut;
mod row;
mod row_iter;
mod rows_iter;
mod view;

pub use col::*;
pub use col_iter::*;
pub use coord::*;
pub use grid::*;
pub use grid_buf::*;
pub use grid_iter::*;
pub use grid_mut::*;
pub use row::*;
pub use row_iter::*;
pub use rows_iter::*;
pub use view::*;

#[test]
fn test() {
    let grid = [
        [0, 1, 2], //
        [3, 4, 5], //
        [6, 7, 8], //
    ];
    dbg!(grid.get_at((2, Wrap(5))));
}
