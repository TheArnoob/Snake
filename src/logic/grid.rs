pub struct Grid<T> {
    data: Vec<T>,
    height: usize,
    width: usize,
}
impl<T: Default + Clone> Grid<T> {
    fn coordinates_to_index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        } else {
            let result = ((x * self.height) + y) as usize;
            return Some(result);
        }
    }
    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        let index = self.coordinates_to_index(x, y)?;
        return self.data.get(index);
    }
    pub fn at_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let index = self.coordinates_to_index(x, y)?;
        return self.data.get_mut(index);
    }

    pub fn new(width: usize, height: usize) -> Grid<T> {
        let mut grid_vec = Vec::new();
        grid_vec.resize((height * width) as usize, T::default());
        Grid {
            data: grid_vec,
            height,
            width,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::logic::grid::Grid;

    #[test]
    fn test() {
        let mut grid = Grid::<u128>::new(4, 2);
        assert_eq!(grid.at(2, 5), None);
        assert_eq!(grid.at(1, 3), None);
        assert_eq!(*grid.at(3, 1).unwrap(), 0);

        *grid.at_mut(3, 1).unwrap() = 4;
        assert_eq!(*grid.at(3, 1).unwrap(), 4);
        assert_eq!(grid.data[7], 4);
        *grid.at_mut(2, 1).unwrap() = 10;
        assert_eq!(grid.data[5], 10);

        assert_eq!(grid.at(4, 0), None);
        assert!(grid.at(3, 0).is_some());
        assert_eq!(grid.at(4, 2), None);
        assert_eq!(grid.at(0, 2), None);
        assert!(grid.at(0, 1).is_some());
        assert_eq!(*grid.at(0, 0).unwrap(), 0)
    }
}
