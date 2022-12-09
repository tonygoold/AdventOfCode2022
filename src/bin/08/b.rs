use aoc::grid::Grid;

fn main() {
    let grid = aoc::read_uint_grid(&aoc::input_arg());
    let (rows, cols) = grid.size();
    let mut score: Grid<usize> = Grid::new(rows, cols);
    grid.enumerate(|(row, col), &height| {
        let mut top: usize = 0;
        let mut i = row;
        let mut j = col;
        while i > 0 {
            i -= 1;
            top += 1;
            if grid[(i, j)] >= height {
                break;
            }
        }
        i = row;
        let mut bottom: usize = 0;
        while i + 1 < rows {
            i += 1;
            bottom += 1;
            if grid[(i, j)] >= height {
                break;
            }
        }
        i = row;
        let mut left: usize = 0;
        while j > 0 {
            j -=1 ;
            left += 1;
            if grid[(i, j)] >= height {
                break;
            }
        }
        j = col;
        let mut right: usize = 0;
        while j + 1 < cols {
            j += 1;
            right += 1;
            if grid[(i, j)] >= height {
                break;
            }
        }
        score[(row, col)] = left * right * top * bottom;
    });
    let mut best: usize = 0;
    for (_, _, &score) in score.iter() {
        if score > best {
            best = score;
        }
    }
    println!("The best score is {}", best);
}
