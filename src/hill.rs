use std::collections::HashSet;
use std::collections::VecDeque;

use crate::grid::Grid;
use crate::point::Point2D;

pub type Coord = Point2D<usize>;

fn char_height(c: char) -> usize {
    match c {
        'S' => 0,
        'E' => 25,
        _ => (c as usize) - ('a' as usize),
    }
}

pub struct Hill {
    grid: Grid<char>,
}

impl Hill {
    pub fn new(grid: Grid<char>) -> Self {
        Self { grid }
    }

    pub fn grid(&self) -> &Grid<char> {
        &self.grid
    }

    pub fn find_path(&self, start_pos: Coord, best: usize) -> Option<Vec<Coord>> {
        let (rows, cols) = self.grid.size();
        let mut backtrack: Grid<Coord> = Grid::new(rows, cols);
        let mut queue: VecDeque<Coord> = VecDeque::new();
        let mut visited: HashSet<Coord> = HashSet::new();
        let (y, x, _) = self.grid.iter().find(|(_, _, &c)| c == 'E')
            .expect("end position not found");
        let end_pos = Coord::new(x, y);
        queue.push_back(start_pos);
        visited.insert(start_pos);
        let mut len: usize = 0;
        let mut at_cur_len: usize = 1;
        let mut at_next_len = 0;
        let mut found = false;
        while let Some(pos) = queue.pop_front() {
            if pos == end_pos {
                found = true;
                break;
            }
            if at_cur_len == 0 {
                len += 1;
                if best != 0 && len >= best {
                    return None;
                }
                at_cur_len = at_next_len;
                at_next_len = 0;
            }
            at_cur_len -= 1;

            let c = self.grid[(pos.y, pos.x)];

            let mut neighbours: Vec<Coord> = Vec::new();
            if pos.x > 0 {
                neighbours.push(Coord::new(pos.x - 1, pos.y));
            }
            if pos.x + 1 < cols {
                neighbours.push(Coord::new(pos.x + 1, pos.y));
            }
            if pos.y > 0 {
                neighbours.push(Coord::new(pos.x, pos.y - 1));
            }
            if pos.y + 1 < rows {
                neighbours.push(Coord::new(pos.x, pos.y + 1));
            }

            let src_height = char_height(c);
            for neighbour in neighbours {
                if visited.contains(&neighbour) {
                    continue;
                }
                let dst = self.grid[(neighbour.y, neighbour.x)];
                let dst_height = char_height(dst);
                if src_height + 1 >= dst_height {
                    backtrack[(neighbour.y, neighbour.x)] = pos;
                    queue.push_back(neighbour);
                    visited.insert(neighbour);
                    at_next_len += 1;
                }
            }
        }
        if ! found {
            return None;
        }

        let mut path: Vec<Coord> = Vec::new();
        let mut pos = end_pos;
        loop {
            if pos == start_pos {
                break;
            }
            let (x, y) = (pos.x, pos.y);
            path.push(pos);
            pos = backtrack[(y, x)];
        }
        Some(path)
    }
}
