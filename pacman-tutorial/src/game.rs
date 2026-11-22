use crate::board::{Board, Tile};
use crate::{Direction, Player, Position};

#[derive(Debug, PartialEq, Eq)]
enum MoveOutcome {
    Blocked,
    Moved,
    CollectedPellet,
}

struct GameState {
    player: Player,
    board: Board,
    score: u32,
}

impl GameState {
    fn new(player: Player, board: Board) -> Self {
        Self {
            player,
            board,
            score: 0,
        }
    }

    fn step(&mut self, direction: Direction) -> MoveOutcome {
        todo!("Implement movement and collision rules")
    }

    fn score(&self) -> u32 {
        self.score
    }

    fn player_position(&self) -> Position {
        self.player.position()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_game() -> GameState {
        let board = Board::new(vec![
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
            vec![
                Tile::Wall,
                Tile::Empty,
                Tile::Pellet,
                Tile::Empty,
                Tile::Wall,
            ],
            vec![Tile::Wall, Tile::Empty, Tile::Wall, Tile::Empty, Tile::Wall],
        ]);

        let player = Player::new(String::from("Pac-Man"), Position { x: 1, y: 1 });

        GameState::new(player, board)
    }

    #[test]
    fn player_cannot_move_into_wall() {
        let mut game = example_game();

        let outcome = game.step(Direction::Up);

        assert_eq!(outcome, MoveOutcome::Blocked);
        assert_eq!(game.player_position(), Position { x: 1, y: 1 });
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn player_collects_pellet() {
        let mut game = example_game();

        let outcome = game.step(Direction::Right);

        assert_eq!(outcome, MoveOutcome::CollectedPellet);
        assert_eq!(game.score(), 10);

        assert_eq!(
            game.board.tile_at(Position { x: 2, y: 1 }),
            Some(&Tile::Empty)
        );
    }

    #[test]
    fn player_moves_to_empty_tile() {
        let mut game = example_game();

        let outcome = game.step(Direction::Down);

        assert_eq!(outcome, MoveOutcome::Moved);
        assert_eq!(game.player_position(), Position { x: 1, y: 2 });
        assert_eq!(game.score(), 0);
    }
}
