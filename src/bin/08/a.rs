use aoc::grid::Grid;

fn main() {
    let grid = aoc::read_uint_grid(&aoc::input_arg());
    let (rows, cols) = grid.size();
    let mut hidden: Grid<bool> = Grid::new(rows, cols);
    for i in 0..rows {
        let mut max_height = 0;
        let grid_row = &grid[i];
        let hidden_row = &mut hidden[i];
        for j in 0..cols {
            if j != 0 && grid_row[j] <= max_height {
                hidden_row[j] = true;
            } else {
                max_height = grid_row[j];
            }
        }
        max_height = 0;
        for j in 1..=cols {
            let j = cols - j;
            if j == cols - 1 || grid_row[j] > max_height {
                hidden_row[j] = false;
                max_height = grid_row[j];
            }
        }
    }
    for j in 0..cols {
        let mut max_height = 0;
        for i in 0..rows {
            let height = grid[(i, j)];
            if i == 0 || height > max_height {
                hidden[(i, j)] = false;
                max_height = height;
            }
        }
        max_height = 0;
        for i in 1..=rows {
            let i = rows - i;
            let height = grid[(i, j)];
            if i == rows - 1 || height > max_height {
                hidden[(i, j)] = false;
                max_height = height;
            }
        }
    }
    let hidden_count: usize = hidden.iter().filter(|(_, _, &hidden)| hidden).count();
    let visible = rows * cols - hidden_count;
    println!("There are {} visible tiles", visible);
    for i in 0..rows {
        let row = hidden[i].iter().map(|hidden| if *hidden { ' ' } else { '#' });
        let row_str: String = row.collect();
        println!("{}", &row_str);
    }
}
