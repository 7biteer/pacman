mod board;
mod game;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Position {
    fn moved(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self {
                y: self.y - 1,
                ..self
            },
            Direction::Down => Self {
                y: self.y + 1,
                ..self
            },
            Direction::Left => Self {
                x: self.x - 1,
                ..self
            },
            Direction::Right => Self {
                x: self.x + 1,
                ..self
            },
        }
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    position: Position,
}

impl Player {
    fn new(name: String, position: Position) -> Self {
        Self { name, position }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn position(&self) -> Position {
        self.position
    }

    fn move_in(&mut self, direction: Direction) {
        self.position = self.position.moved(direction);
    }
}

fn main() {
    let start = Position { x: 4, y: 3 };

    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let next = start.moved(direction);
        println!("Moving {:?} from {:?} to {:?}", direction, start, next);
    }

    let mut player = Player::new(String::from("Pac-Man"), Position { x: 4, y: 3 });

    let player_name  = player.name.as_str();

    player.position = player.position.moved(Direction::Right);
    
    println!("{player_name}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_right() {
        let start = Position { x: 4, y: 3 };
        let next = start.moved(Direction::Right);
        assert_eq!(next, Position { x: 5, y: 3 });
    }

    #[test]
    fn moves_down() {
        let start = Position { x: 4, y: 3 };
        let next = start.moved(Direction::Down);
        assert_eq!(next, Position { x: 4, y: 4 });
    }

    #[test]
    fn moves_left() {
        let start = Position { x: 4, y: 3 };
        let next = start.moved(Direction::Left);
        assert_eq!(next, Position { x: 3, y: 3 });
    }

    #[test]
    fn moves_up() {
        let start = Position { x: 4, y: 3 };
        let next = start.moved(Direction::Up);
        assert_eq!(next, Position { x: 4, y: 2 });
    }

    #[test]
    fn movement_does_not_mutate_original() {
        let start = Position { x: 4, y: 3 };
        let _next = start.moved(Direction::Left);
        assert_eq!(start, Position { x: 4, y: 3 });
    }

    #[test]
    fn player_moves_right() {
        let mut player = Player::new(String::from("Pac-Man"), Position { x: 4, y: 3 });

        player.move_in(Direction::Right);

        assert_eq!(player.position(), Position { x: 5, y: 3 });
    }
}

#[cfg(test)]
mod tests_02 {
    use super::*;

    #[test]
    fn player_name_is_accessible_without_cloning() {
        let player = Player::new(String::from("Pac-Man"), Position { x: 4, y: 3 });
        let player_name: &str = player.name();
        assert_eq!(player_name, "Pac-Man");
        assert_eq!(player.position(), Position { x: 4, y: 3 });
    }

    #[test]
    fn player_can_move_multiple_times() {
        let mut player = Player::new(String::from("Pac-Man"), Position { x: 4, y: 3 });
        player.move_in(Direction::Right);
        player.move_in(Direction::Right);
        player.move_in(Direction::Down);
        player.move_in(Direction::Left);
        assert_eq!(player.position(), Position { x: 5, y: 4 });
    }

    #[test]
    fn player_can_move_after_name_borrow_ends() {
        let mut player = Player::new(String::from("Pac-Man"), Position { x: 4, y: 3 });
        let player_name: &str = player.name();
        assert_eq!(player_name, "Pac-Man");
        player.move_in(Direction::Up);
        assert_eq!(player.position(), Position { x: 4, y: 2 });
    }
}
