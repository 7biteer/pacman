use crate::board::Tile;

fn tile_to_char(tile: Tile) -> char {
    match tile {
        Tile::Wall => '#',
        Tile::Empty => ' ',
        Tile::Pellet => '.',
    }
}

fn render_row(row: &[Tile]) -> String {
    row.iter().map(|tile| tile_to_char(*tile)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_row_correctly() {
        let row = [Tile::Wall, Tile::Pellet, Tile::Empty, Tile::Wall];

        assert_eq!(render_row(&row), "#. #");
    }

    #[test]
    fn renders_empty_row() {
        let row: [Tile; 0] = [];

        assert_eq!(render_row(&row), "");
    }
}
