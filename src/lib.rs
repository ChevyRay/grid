//mod coord;
mod grid;
mod grid_buf;
//mod region;
mod grid_mut;
mod iter;
mod iter_mut;
mod view;
mod view_mut;

//pub use coord::*;
pub use grid::*;
pub use grid_buf::*;
pub use grid_mut::*;
pub use iter::*;
pub use iter_mut::*;
pub use view_mut::*;
//ub use region::*;
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

    let mut data = vec![0usize; 25];
    let mut grid = GridBuf::with_store(5, 5, data.as_mut_slice());
    //let mut grid = BufGrid::<usize>::new(5, 5);
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            *grid.get_mut(x, y).unwrap() = y * grid.width() + x;
        }
    }

    display(&grid);

    //let view = grid.view(0, 0, 3, 3).unwrap();
    let view = grid.view(0, 0, 3, 3);

    display(&view);

    let view2 = view.view(1, 1, 2, 2);

    display(&view2);
}
