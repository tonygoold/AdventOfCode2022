use aoc::hill::{Coord, Hill};

fn main() {
    let grid = aoc::read_char_grid(&aoc::input_arg());

    let starts: Vec<Coord> = grid.iter()
        .filter(|(_, _, &c)| c == 'S' || c == 'a')
        .map(|(y, x, _)| Coord::new(x, y))
        .collect();
    let hill = Hill::new(grid);
    let mut min_len: usize = 0;
    for start_pos in starts {
        if let Some(path) = hill.find_path(start_pos, min_len) {
            let mut plen = path.len();
            // The last 'a' encountered is de facto a shorter route
            if let Some((index, _)) = path.iter().enumerate().rfind(|(_, coord)| {
                hill.grid()[(coord.y, coord.x)] == 'a'
            }) {
                plen -= index;
            }
            if min_len == 0 || plen < min_len {
                min_len = plen;
            }
        }
    }

    println!("The length of the shortest path is {}", min_len);
}
