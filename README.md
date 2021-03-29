# Grid
A WIP Rust library with 2D grid data structures. Not ready for public use, these are completely untested and mostly just me experimenting with an API structure for this. They also use unsafe code to speed up some operations, but I haven't fully tested these to guarantee their safety in some circumstances.

There are currently 3 different types of grids, each which implements the `Grid` trait:

Grid Type | Description
---|---
`VecGrid<T>` | a resizable grid with data stored in a `Vec<T>`
`ConstGrid<T, W, H>` | a constant-size grid of dimensions `W x H`
`SparseGrid<T>` | a grid optimized for storing sparse data over large areas 

The `Grid` trait also implements a lot of functions for manipulating the grids.
You can copy from one to another, fill in regions, check conditional bounds, and
iterate over regions of them.

There are `_unchecked` versions of most of the functions, which uses the `get_unchecked`
variations of the getter methods for ignoring bounds-checks on the algorithms. With
`VecGrid` and `ConstGrid` specifically, any of these that operate over
rectangular regions *should* be perfectly safe, since only the region where they provided
rectangles overlap with the grid should be scanned. But with `SparseGrid`,
any cells that have not been specifically filled with values are invalid, and calling
`get_unchecked` on those positions is undefined behavior.

I was thinking that these rect-scanning algorithms could have their `unsafe`
removed if `SparseGrid` specifically just panicked if you tried to retrieve
from an invalid location, but that seems un-idiomatic and also confusing to
the end-user. Also, this doesn't account for other user-created types that
might implement `Grid`, which the algorithms cannot account for.

Anyway, I'm still working on this, and I'm hoping to add more useful
functionality to them, as well as writing a bunch of generic tests to check the
correctness of each of the grid types, but I'll do that once I'm happy with
the API I've laid out.