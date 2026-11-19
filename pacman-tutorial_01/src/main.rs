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
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
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
    fn moves_up() {
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
    fn moves_down() {
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
}
