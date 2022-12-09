use aoc::snake::{Movement, Snake};

fn main() {
    let moves: Result<Vec<Movement>, _> = aoc::read_lines(&aoc::input_arg())
        .map(|line| line.parse::<Movement>()).collect();
    let moves = moves.expect("failed to parse moves");
    let mut snake = Snake::default();
    snake.apply_moves(&moves);
    let visited = snake.tails_visited[0].len();
    println!("The tail visited {} locations", visited);
}
