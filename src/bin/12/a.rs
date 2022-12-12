use aoc::hill::{Coord, Hill};

fn main() {
    let grid = aoc::read_char_grid(&aoc::input_arg());

    let (y, x, _) = grid.iter().find(|(_, _, &c)| c == 'S')
        .expect("start position not found");
    let start_pos = Coord::new(x, y);
    let hill = Hill::new(grid);
    let path = hill.find_path(start_pos, 0)
        .expect("Could not find a solution");

    println!("The length of the path is {}", path.len());
}
