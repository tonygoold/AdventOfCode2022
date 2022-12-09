use std::collections::HashSet;
use std::str::FromStr;

use crate::point::Point2D;

#[derive(Debug, Copy, Clone)]
pub enum ParseMovementError {
    InvalidDirection,
    InvalidDistance,
}

#[derive(Debug, Copy, Clone)]
pub enum Movement {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let dir = parts.next().ok_or(Self::Err::InvalidDirection)?;

        let dist = parts.next().ok_or(Self::Err::InvalidDistance)?;
        let dist = dist.parse::<usize>().map_err(|_| Self::Err::InvalidDistance)?;
        
        match dir {
            "L" => Ok(Movement::Left(dist)),
            "R" => Ok(Movement::Right(dist)),
            "U" => Ok(Movement::Up(dist)),
            "D" => Ok(Movement::Down(dist)),
            _ => Err(Self::Err::InvalidDirection),
        }
    }
}

pub struct Snake {
    pub head: Point2D<isize>,
    pub tails: Vec<Point2D<isize>>,
    pub tails_visited: Vec<HashSet<Point2D<isize>>>,
}

impl Snake {
    pub fn new(head: Point2D<isize>, num_tails: usize) -> Self {
        let mut tails = Vec::new();
        let mut tails_visited = Vec::new();
        (0..num_tails).for_each(|_| {
            tails.push(head);
            let mut visited = HashSet::new();
            visited.insert(head);
            tails_visited.push(visited);
        });
        Self { head, tails, tails_visited }
    }

    pub fn move_left(&mut self) {
        self.head.x -= 1;
        self.update_tails();
    }

    pub fn move_right(&mut self) {
        self.head.x += 1;
        self.update_tails();
    }

    pub fn move_up(&mut self) {
        self.head.y += 1;
        self.update_tails();
    }

    pub fn move_down(&mut self) {
        self.head.y -= 1;
        self.update_tails();
    }

    pub fn apply_moves(&mut self, moves: &[Movement]) {
        for mv in moves {
            match mv {
                Movement::Left(x) => (0..*x).for_each(|_| self.move_left()),
                Movement::Right(x) => (0..*x).for_each(|_| self.move_right()),
                Movement::Up(y) => (0..*y).for_each(|_| self.move_up()),
                Movement::Down(y) => (0..*y).for_each(|_| self.move_down()),
            }
        }
    }

    fn update_tails(&mut self) {
        (0..self.tails.len()).for_each(|i| self.update_tail(i));
    }

    fn update_tail(&mut self, index: usize) {
        let head = if index > 0 { &self.tails[index - 1] } else { &self.head };
        let tail = &self.tails[index];
        let dist = Point2D::new(head.x - tail.x, head.y - tail.y);
        let adx = dist.x.abs();
        let ady = dist.y.abs();
        if adx > 1 || ady > 1 {
            // On a diagonal separation, both coordinates change
            let threshold = if adx > 0 && ady > 0 { 0 } else { 1 };
            let dx = if dist.x > threshold { 1 } else if dist.x < -threshold { -1 } else { 0 };
            let dy = if dist.y > threshold { 1 } else if dist.y < -threshold { -1 } else { 0 };
            self.tails[index].x += dx;
            self.tails[index].y += dy;
        }
        self.tails_visited[index].insert(self.tails[index]);
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self::new(Point2D::default(), 1)
    }
}

#[cfg(test)]
mod test {
    use crate::point::Point2D;
    use super::Snake;

    #[test]
    fn separated_left() {
        let mut snake = Snake::default();
        snake.move_left();
        assert_eq!(snake.tails[0], Point2D::default());
        snake.move_left();
        assert_eq!(snake.tails[0], Point2D::new(-1, 0));
    }

    #[test]
    fn separated_right() {
        let mut snake = Snake::default();
        snake.move_right();
        assert_eq!(snake.tails[0], Point2D::default());
        snake.move_right();
        assert_eq!(snake.tails[0], Point2D::new(1, 0));
    }

    #[test]
    fn separated_down() {
        let mut snake = Snake::default();
        snake.move_down();
        assert_eq!(snake.tails[0], Point2D::default());
        snake.move_down();
        assert_eq!(snake.tails[0], Point2D::new(0, -1));
    }

    #[test]
    fn separated_up() {
        let mut snake = Snake::default();
        snake.move_up();
        assert_eq!(snake.tails[0], Point2D::default());
        snake.move_up();
        assert_eq!(snake.tails[0], Point2D::new(0, 1));
    }

    #[test]
    fn separated_diag_up() {
        let mut snake = Snake::default();
        snake.move_right();
        snake.move_up();
        assert_eq!(snake.tails[0], Point2D::default());
        snake.move_up();
        assert_eq!(snake.tails[0], Point2D::new(1, 1));
    }

    #[test]
    fn separated_diag_right() {
        let mut snake = Snake::default();
        snake.move_right();
        snake.move_up();
        assert_eq!(snake.tails[0], Point2D::default());
        snake.move_right();
        assert_eq!(snake.tails[0], Point2D::new(1, 1));
    }

    #[test]
    fn full_path() {
        let mut snake = Snake::default();
        assert_eq!(snake.tails[0], Point2D::default());
        // R 4
        snake.move_right();
        assert_eq!(snake.tails[0], Point2D::default());
        snake.move_right();
        assert_eq!(snake.tails[0], Point2D::new(1, 0));
        snake.move_right();
        assert_eq!(snake.tails[0], Point2D::new(2, 0));
        snake.move_right();
        assert_eq!(snake.tails[0], Point2D::new(3, 0));
        // U 4
        snake.move_up();
        assert_eq!(snake.tails[0], Point2D::new(3, 0));
        snake.move_up();
        assert_eq!(snake.tails[0], Point2D::new(4, 1));
        snake.move_up();
        assert_eq!(snake.tails[0], Point2D::new(4, 2));
        snake.move_up();
        assert_eq!(snake.tails[0], Point2D::new(4, 3));
    }
}
