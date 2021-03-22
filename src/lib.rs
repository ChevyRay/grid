mod grid;

pub use grid::Grid;

pub struct Item(bool);

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let mut arr: Grid<Item> = Grid::new_with(2, 2, || Item(false));
        let mut iter = arr.iter_mut();
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        *a.1 = Item(true);
        *b.1 = Item(true);
    }
}
