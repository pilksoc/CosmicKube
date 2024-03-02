use std::collections::BTreeMap;
use crate::space::{ Space, SpaceKind };
use crate::Coordinate;

/// A grid to represent the playing area. Only saves spaces which aren't empty.
pub struct Grid {
    spaces: BTreeMap<Coordinate, Space>,
    width: u64,
    height: u64,
}
impl Grid {
    pub fn new(width: u64, height: u64) -> Grid {
        Grid {
            spaces: BTreeMap::new(),
            width,
            height,
        }
    }
    pub fn from_spaces(spaces: Vec<Space>, width: u64, height: u64) -> Grid {
        let mut space_tree: BTreeMap<Coordinate, Space> = BTreeMap::new();
        for space in spaces {
            space_tree.insert(space.coordinate, space);
        }
        Grid {
            spaces: space_tree,
            width,
            height,
        }

    }
    pub fn insert(&mut self, space: Space) {
        self.spaces.insert(space.coordinate, space);
    }
    /// Checks that a coordinate is not beyond the bounds of the grid.
    pub fn in_bounds(&self, coordinate: [u64; 2]) -> bool {
        coordinate[0] < self.width && coordinate[1] < self.height
    }

    /// Looks for a space with the given coordinates in a grid. If there is no space (it is an empty space), then it returns [`std::option::Option::None`].
    ///
    /// # Example
    /// ```rust
    /// use cosmic_kube::grid::Grid;
    /// use cosmic_kube::space::{ Space, SpaceKind };
    /// 
    /// let grid = Grid::from_spaces(
    ///     vec![
    ///         Space::new([0, 2], SpaceKind::EmptySpace),
    ///         Space::new([1, 0], SpaceKind::EmptySpace),
    ///     ],
    ///     3,
    ///     3,
    /// );
    /// assert_eq!(Some(&Space::new([1, 0], SpaceKind::EmptySpace)), grid.get_space([1, 0]));
    /// assert_eq!(None, grid.get_space([2, 2]));
    /// ```
    pub fn get_space(&self, coordinate: Coordinate) -> Option<&Space> {
        self.spaces.get(&coordinate)
    }

    fn neighbour_coords_in_bounds(&self, coordinate: Coordinate) -> Vec<Coordinate> {
        let coordinates: [[Option<u64>; 2]; 8] = [
            [coordinate[0].checked_add(1), Some(coordinate[1])],
            [coordinate[0].checked_add(1), coordinate[1].checked_add(1)],
            [Some(coordinate[0]), coordinate[1].checked_add(1)],
            [coordinate[0].checked_sub(1), coordinate[1].checked_add(1)],
            [coordinate[0].checked_sub(1), Some(coordinate[1])],
            [coordinate[0].checked_sub(1), coordinate[1].checked_sub(1)],
            [Some(coordinate[0]), coordinate[1].checked_sub(1)],
            [coordinate[0].checked_add(1), coordinate[1].checked_sub(1)],
        ];
        coordinates.iter()
            .filter_map(|c| match *c {
                [Some(x), Some(y)] => Some([x, y]),
                _ => None,
            })
            .filter(|coord| self.in_bounds(*coord))
            .collect::<Vec<_>>()
    }

    /// Returns neighbours which are in the grid *and* which aren't [`SpaceKind::EmptySpace`]s.
    pub fn get_nonempty_neighbours(&self, coordinate: Coordinate) -> Vec<&Space> {
        self.neighbour_coords_in_bounds(coordinate).iter()
            .map(|coord| self.get_space(*coord))
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::space::{Space, SpaceKind};
    fn grid_3x3() -> Grid {
        let mut grid = Grid::new(3, 3);
        let spaces = [
            Space::new([0, 0], SpaceKind::EmptySpace),
            Space::new([0, 1], SpaceKind::EmptySpace),
            Space::new([0, 2], SpaceKind::EmptySpace),
            Space::new([1, 0], SpaceKind::EmptySpace),
            Space::new([2, 2], SpaceKind::EmptySpace),
        ];
        for space in spaces {
            grid.insert(space);
        }
        grid
    }
    #[test]
    fn coords_in_bounds() {
        let grid = grid_3x3();
        let neighbours = grid.get_nonempty_neighbours([0, 2]);
        assert_eq!(vec![&Space::new([0, 1], SpaceKind::EmptySpace)], neighbours);
    }
}
