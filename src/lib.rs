mod grid;
mod grid_buf;
mod grid_iter;
mod grid_iter_mut;
mod grid_mut;
mod row;
mod row_iter;
mod rows;
mod view;

pub use grid::*;
pub use grid_buf::*;
pub use grid_iter::*;
pub use grid_iter_mut::*;
pub use grid_mut::*;
pub use row::*;
pub use row_iter::*;
pub use rows::*;
pub use view::*;

// Wrapped<Coord<usize>>
// wrapped((0, 0))
// coord(0, 0)
// icoord(0, 0)

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

    display(&targ);
}
