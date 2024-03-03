use std::collections::BTreeMap;
use crate::space::{Space, SpaceKind};
use crate::vecmap::VecMap;
use crate::Coordinate;
use thiserror::Error;
use uuid::Uuid;

fn distance([a, b]: Coordinate, [x, y]: Coordinate) -> u64 {
    (a.abs_diff(x)).max(b.abs_diff(y))
}

#[derive(Debug, Clone, Copy)]
/// The direction in which to grow. For example, going from 3 to 9 would give a `GrowDirection::Expand`.
enum GrowDirection {
    Shrink,
    Expand,
}

/// An error when trying to expand or shrink the grid.
#[derive(Error, Debug)]
pub enum ResizeError {
    #[error("At least one Kube which was formerly in the grid would not be in the grid after shrinking.")]
    KubeWillBeOutOfBounds,
    #[error("Grid width or height will exceed `u64::MAX`.")]
    GridTooBig,
    #[error("Grid width or height will be less than zero.")]
    GridTooSmall,
}

/// A grid to represent the playing area. Only saves spaces which aren't empty.
#[derive(Debug, PartialEq)]
pub struct Grid {
    spaces: BTreeMap<Coordinate, Space>,
    width: u64,
    height: u64,
    players: VecMap<Uuid, Coordinate>,
}
impl Grid {
    pub fn new(width: u64, height: u64) -> Grid {
        Grid {
            spaces: BTreeMap::new(),
            width,
            height,
            players: VecMap::new(),
        }
    }
    pub fn from_spaces(spaces: Vec<Space>, width: u64, height: u64) -> Grid {
        let mut space_tree: BTreeMap<Coordinate, Space> = BTreeMap::new();
        let mut players: VecMap<Uuid, Coordinate> = VecMap::new();
        for space in spaces {
            if let SpaceKind::Player(player) = &space.contains {
                players.update(player.uuid, space.coordinate);
            }
            space_tree.insert(space.coordinate, space);
        }
        Grid {
            spaces: space_tree,
            width,
            height,
            players,
        }

    }

    /// Adds a new space to the grid. In the future, this may return an [`std::result::Result::Err`] if the space is of type [`crate::space::SpaceKind::EmptySpace`].
    pub fn insert(&mut self, space: Space) {
        if let SpaceKind::Player(player) = &space.contains {
            self.players.update(player.uuid, space.coordinate);
        }
        self.spaces.insert(space.coordinate, space);
    }
    /// Removes the space at the given coordinate. If there is no recorded space there, then this returns [`std::option::Option::None`].
    pub fn remove(&mut self, coordinate: Coordinate) -> Option<Space> {
        if let Some(space) = self.spaces.remove(&coordinate) {
            if let SpaceKind::Player(player) = &space.contains {
                let _ = self.players.remove(&player.uuid);
            }
            Some(space)
        }
        else {
            None
        }
        
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

    /// Gets the neighbours that are *n* squares away.
    ///
    /// In other words, this will look at all the rings which are 1, 2, …, n squares away from the coordinate. It will return any squares found in these rings.
    pub fn get_neighbours_n_away(&self, coordinate: Coordinate, n: u64) -> Vec<&Space> {
        let mut coords: Vec<Coordinate> = Vec::new();
        let mut stack: Vec<Coordinate> = self.neighbour_coords_in_bounds(coordinate);
        while let Some(coord) = stack.pop() {
            if coords.contains(&coord) {
                continue;
            }
            coords.push(coord);
            stack.extend(
            self.neighbour_coords_in_bounds(coord)
                .iter()
                .filter(|c|
                    distance(coordinate, **c) <= n && distance(coordinate, **c) > distance(coord, **c)
                )
            );
        }
        let mut to_return: Vec<&Space> = Vec::new();
        for coord in coords {
            if let Some(space) = self.spaces.get(&coord) {
                to_return.push(space)
            }
        }
        to_return
    }

    /// Gets all the neighbours (directly adjacent squares incl. diagonally) of the coordinate if they're in bounds.
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

    /// Returns neighbours (adjacent incl. diagonally) which are in the grid *and* which aren't [`crate::space::SpaceKind::EmptySpace`]s.
    pub fn get_nonempty_neighbours(&self, coordinate: Coordinate) -> Vec<&Space> {
        self.neighbour_coords_in_bounds(coordinate).iter()
            .map(|coord| self.get_space(*coord))
            .flatten()
            .collect()
    }

    /// Returns the grid size in the format `[width, height]`.
    fn get_grid_size(&self) -> [u64; 2] {
        [self.width, self.height]
    }

    fn change_grid_by_rings(&mut self, rings_to_change_by: u64, direction: GrowDirection) -> Result<(), ResizeError> {
        // If the specified size is the same as the current size, no need to do anything.
        if rings_to_change_by == 0 {
            return Ok(());
        }
        let Some(double_rings) = rings_to_change_by.checked_mul(2) else {
            return Err(ResizeError::GridTooBig)
        };
        let Some(new_width) = (match direction {
            GrowDirection::Expand => self.width.checked_add(double_rings),
            GrowDirection::Shrink => self.width.checked_sub(double_rings),
        }) else {
            return Err(ResizeError::GridTooBig);
        };
        let Some(new_height) = (match direction {
            GrowDirection::Expand => self.height.checked_add(double_rings),
            GrowDirection::Shrink => self.height.checked_sub(double_rings),
        }) else {
            return Err(ResizeError::GridTooBig);
        };

        // Careful! If the map is big, this may use a lot of memory!
        let mut spaces_in_map: Vec<Space> = self.spaces.clone().into_values().collect();
        for space in &mut spaces_in_map {
            space.coordinate[0] = match direction {
                GrowDirection::Expand => {
                    match space.coordinate[0].checked_add(rings_to_change_by) {
                        Some(n) => n,
                        None => return Err(ResizeError::KubeWillBeOutOfBounds),
                    }
                },
                GrowDirection::Shrink => {
                    match space.coordinate[0].checked_sub(rings_to_change_by) {
                        Some(n) => n,
                        None => return Err(ResizeError::KubeWillBeOutOfBounds),
                    }
                }
            };
            space.coordinate[1] = match direction {
                GrowDirection::Expand => {
                    match space.coordinate[1].checked_add(rings_to_change_by) {
                        Some(n) => n,
                        None => return Err(ResizeError::KubeWillBeOutOfBounds),
                    }
                },
                GrowDirection::Shrink => {
                    match space.coordinate[1].checked_sub(rings_to_change_by) {
                        Some(n) => n,
                        None => return Err(ResizeError::KubeWillBeOutOfBounds),
                    }
                }
            };
        }
        self.spaces = BTreeMap::new();
        for space in spaces_in_map {
            self.insert(space);
        }
        self.width = new_width;
        self.height = new_height;
        Ok(())

    }

    /// This will expand the grid size and change the coordinates of the respective kubes.
    ///
    /// The change in size can be thought of as "rings of squares" to be added around the outside of the grid. So if the grid used to be a 2×2 grid, adding a ring of squares around the outside will give a 4×4 square.
    ///
    /// <div class="warning">Warning! This function may use a lot of memory! When resizing the grid, the program needs to copy all of the grid's contents to a new `Vec`. If the grid is densely populated then take care when calling it.</div>
    ///
    /// # Errors
    ///
    /// Will fail if the resize request is invalid, if the new grid will be too big, (bigger than 2^64 - 1).
    pub fn expand_grid(&mut self, rings_to_add: u64) -> Result<(), ResizeError> {
        self.change_grid_by_rings(rings_to_add, GrowDirection::Expand)
    }

    /// Like [`crate::grid::Grid::expand_grid`], but instead of expanding, this shrinks.
    ///
    /// <div class="warning">Warning! This function may use a lot of memory! When resizing the grid, the program needs to copy all of the grid's contents to a new `Vec`. If the grid is densely populated then take care when calling it.</div>
    ///
    /// # Errors
    ///
    /// Will fail if the resize request is invalid.
    /// - If the new grid will be too small, (smaller than 1).
    /// - If there is a Kube near the edge such that it would be "cut off" when the rings are removed.
    pub fn shrink_grid(&mut self, rings_to_shrink_by: u64) -> Result<(), ResizeError> {
        self.change_grid_by_rings(rings_to_shrink_by, GrowDirection::Shrink)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::space::{Space, SpaceKind};
    fn grid_2x2() -> Grid {
        let mut grid = Grid::new(2, 2);
        let spaces = [
            Space::new([0, 0], SpaceKind::EmptySpace),
            Space::new([0, 1], SpaceKind::EmptySpace),
            Space::new([1, 1], SpaceKind::EmptySpace),
        ];
        for space in spaces {
            grid.insert(space);
        }
        grid
    }
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

    #[test]
    fn increase_2_to_4() {
        let mut grid = grid_2x2();
        let _ = grid.expand_grid(1);
        let mut expected_grid = Grid::new(4, 4);
        let spaces = [
            Space::new([1, 1], SpaceKind::EmptySpace),
            Space::new([1, 2], SpaceKind::EmptySpace),
            Space::new([2, 2], SpaceKind::EmptySpace),
        ];
        for space in spaces {
            expected_grid.insert(space);
        }
        assert_eq!(expected_grid, grid);
    }
    #[test]
    fn decrease_from_3_to_1() {
        let mut grid = Grid::new(3, 3);
        grid.insert(Space::new([1, 1], SpaceKind::EmptySpace));
        let _ = grid.shrink_grid(1);
        let mut expected_grid = Grid::new(1, 1);
        let spaces = [
            Space::new([0, 0], SpaceKind::EmptySpace),
        ];
        for space in spaces {
            expected_grid.insert(space);
        }
        assert_eq!(expected_grid, grid);
    }
    #[test]
    fn fail_on_invalid_grid_size_change() {
        let mut grid = grid_3x3();
        let shrink_res = grid.shrink_grid(1);
        assert!(shrink_res.is_err());
        let expand_res = grid.expand_grid(u64::MAX - 1);
        assert!(expand_res.is_err());
    }

    #[test]
    fn find_neighbours_2_away() {
        let grid = grid_3x3();
        let neighbours_2_away = grid.get_neighbours_n_away([1, 1], 2).sort();
        assert_eq!(
            grid.spaces.values().collect::<Vec<&Space>>().sort(),
            neighbours_2_away,
        )
    }
}
