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
        let current = self.player.position();
        let next = current.moved(direction);

        let tile = self.board.tile_at(next).copied();

        match tile {
            Some(Tile::Wall) | None => MoveOutcome::Blocked,
            Some(Tile::Empty) => {
                self.player.move_in(direction);
                MoveOutcome::Moved
            }
            Some(Tile::Pellet) => {
                if self.board.collect_pellet(next) {
                    self.player.move_in(direction);
                    self.score += 10;
                    MoveOutcome::CollectedPellet
                } else {
                    MoveOutcome::CollectedPellet
                }
            }
        }
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

    #[test]
    fn pellet_can_only_be_collected_once() {
        let mut game = example_game();

        let first = game.step(Direction::Right);

        assert_eq!(first, MoveOutcome::CollectedPellet);

        assert_eq!(game.score(), 10);

        game.step(Direction::Left);

        let second = game.step(Direction::Right);

        assert_eq!(second, MoveOutcome::Moved);
        assert_eq!(game.score(), 10);
    }

    #[test]
    fn player_cannot_move_outside_board() {
        let board = Board::new(vec![
            vec![Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty],
        ]);
    
        let player = Player::new(
            String::from("Pac-Man"),
            Position { x: 0, y: 0 },
        );
    
        let mut game = GameState::new(player, board);
    
        let result = game.step(Direction::Left);
    
        assert_eq!(result, MoveOutcome::Blocked);
        assert_eq!(
            game.player_position(),
            Position { x: 0, y: 0 }
        );
        assert_eq!(game.score(), 0);
    }
}
