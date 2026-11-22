use crate::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Empty,
    Pellet,
}

pub struct Board {
    rows: Vec<Vec<Tile>>,
}

impl Board {
    pub(crate) fn new(rows: Vec<Vec<Tile>>) -> Self {
        Self { rows }
    }

    pub(crate) fn tile_at(&self, position: Position) -> Option<&Tile> {
        let x = usize::try_from(position.x).ok()?;
        let y = usize::try_from(position.y).ok()?;

        self.rows.get(y)?.get(x)
    }

    fn tile_at_mut(&mut self, position: Position) -> Option<&mut Tile> {
        let x = usize::try_from(position.x).ok()?;
        let y = usize::try_from(position.y).ok()?;

        self.rows.get_mut(y)?.get_mut(x)
    }

    fn collect_pellet(&mut self, position: Position) -> bool {
        match self.tile_at_mut(position) {
            Some(tile) if *tile == Tile::Pellet => {
                *tile = Tile::Empty;
                true
            }
            _ => false,
        }
    }
}

fn select_tile<'a>(first: &'a Tile, second: &'a Tile, choose_first: bool) -> &'a Tile {
    if choose_first { first } else { second }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_board() -> Board {
        Board::new(vec![
            vec![Tile::Wall, Tile::Pellet, Tile::Empty],
            vec![Tile::Empty, Tile::Wall, Tile::Pellet],
        ])
    }

    #[test]
    fn returns_tile_at_valid_position() {
        let board = example_board();

        let tile = board.tile_at(Position { x: 1, y: 0 });

        assert_eq!(tile, Some(&Tile::Pellet));
    }

    #[test]
    fn returns_none_for_negative_coordinates() {
        let board = example_board();

        let tile = board.tile_at(Position { x: -1, y: 0 });

        assert_eq!(tile, None);
    }

    #[test]
    fn returns_none_for_out_of_bounds_position() {
        let board = example_board();

        let tile = board.tile_at(Position { x: 10, y: 10 });

        assert_eq!(tile, None);
    }

    #[test]
    fn selects_second_tile() {
        let first = Tile::Wall;
        let second = Tile::Pellet;

        let result = select_tile(&first, &second, false);

        assert_eq!(*result, Tile::Pellet);
        assert!(std::ptr::eq(result, &second));
    }

    #[test]
    fn collecting_pellet_removes_it_from_board() {
        let mut board = Board::new(vec![vec![Tile::Wall, Tile::Pellet]]);

        let position = Position { x: 1, y: 0 };

        let collected = board.collect_pellet(position);

        assert!(collected);
        assert_eq!(board.tile_at(position), Some(&Tile::Empty));

        assert!(!board.collect_pellet(position));
    }
}
