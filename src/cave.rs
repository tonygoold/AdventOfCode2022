use std::str::FromStr;

use crate::point::Point2D;

// For this problem, positive Y is treated as down
pub type Point = Point2D<isize>;

#[derive(Debug, Copy, Clone)]
pub enum Movement {
    Down,
    DownLeft,
    DownRight,
    Stop,
    Escape,
}

#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    NotEnoughPoints,
    NotEnoughCoords,
    InvalidNumber,
    NotAxisAligned,
}

pub struct Path {
    points: Vec<Point>,
}

impl Path {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    pub fn contains(&self, p: Point) -> bool {
        for (p1, p2) in self.points.iter().zip(self.points.iter().skip(1)) {
            if p.x == p1.x && p1.x == p2.x {
                if p1.y < p2.y {
                    if (p1.y..=p2.y).contains(&p.y) {
                        return true;
                    }
                } else if (p2.y..=p1.y).contains(&p.y) {
                    return true;
                }
            } else if p.y == p1.y && p1.y == p2.y {
                if p1.x < p2.x {
                    if (p1.x..=p2.x).contains(&p.x) {
                        return true;
                    }
                } else if (p2.x..=p1.x).contains(&p.x) {
                    return true;
                }
            }
        }
        false
    }

    pub fn to_points(&self) -> Vec<Point> {
        let mut ps: Vec<Point> = Vec::new();
        for (i, (p1, p2)) in self.points.iter().zip(self.points.iter().skip(1)).enumerate() {
            if i == 0 {
                ps.push(*p1);
            }
            match (p1.x == p2.x, p1.x < p2.x) {
                (true, _) => {
                    if p1.y < p2.y {
                        ps.extend((p1.y+1..=p2.y).map(|y| Point::new(p1.x, y)));
                    } else {
                        ps.extend((p2.y+1..=p1.y).map(|y| Point::new(p1.x, y)));
                    }
                }
                (_, true) => {
                    ps.extend((p1.x+1..=p2.x).map(|x| Point::new(x, p1.y)));
                }
                _ => {
                    ps.extend((p2.x+1..=p1.x).map(|x| Point::new(x, p1.y)));
                }
            }
        };
        ps
    }
}

impl FromStr for Path {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let point_strs = s.split(" -> ");
        let mut points: Vec<Point> = Vec::new();
        for point_str in point_strs {
            let mut coord_strs = point_str.split(',');
            let x: isize = coord_strs.next().ok_or(Self::Err::NotEnoughCoords)?
                .parse().map_err(|_| Self::Err::InvalidNumber)?;
            let y: isize = coord_strs.next().ok_or(Self::Err::NotEnoughCoords)?
                .parse().map_err(|_| Self::Err::InvalidNumber)?;
            points.push(Point::new(x, y));
        }
        if points.len() < 2 {
            return Err(Self::Err::NotEnoughPoints);
        }
        // Sanity check
        for (p1, p2) in points.iter().zip(points.iter().skip(1)) {
            if p1.x != p2.x && p1.y != p2.y {
                return Err(Self::Err::NotAxisAligned);
            }
        }
        Ok(Path::new(points))
    }
}

pub struct Cave {
    paths: Vec<Path>,
    sand: Vec<Point>,
    cur_sand: Option<Point>,
    floor: isize,
    infinite: bool,
}

impl Cave {
    pub fn new(paths: Vec<Path>, infinite: bool) -> Self {
        let mut floor = 0;
        for path in paths.iter() {
            for p in path.points.iter() {
                if p.y > floor {
                    floor = p.y
                }
            }
        }
        if !infinite {
            floor += 2;
        }
        Self {
            paths,
            sand: Vec::new(),
            cur_sand: None,
            floor,
            infinite,
        }
    }

    fn intersects(&self, p: Point) -> bool {
        if !self.infinite && p.y == self.floor {
            true
        } else {
            self.paths.iter().any(|path| path.contains(p))
        }
    }

    pub fn tick(&mut self) -> Movement {
        let p = match self.cur_sand {
            Some(p) => p,
            None => {
                self.cur_sand = Some(Point::new(500, 0));
                return Movement::Down;
            }
        };
        if self.infinite && p.y >= self.floor {
            self.cur_sand = None;
            return Movement::Escape;
        }
        let mut q = p;
        q.y += 1;
        if !self.intersects(q) {
            self.cur_sand = Some(q);
            return Movement::Down;
        }
        q.x -= 1;
        if !self.intersects(q) {
            self.cur_sand = Some(q);
            return Movement::DownLeft;
        }
        q.x += 2;
        if !self.intersects(q) {
            self.cur_sand = Some(q);
            return Movement::DownRight;
        }
        self.sand.push(p);
        self.cur_sand = None;
        if !self.infinite && p.x == 500 && p.y == 0 {
            return Movement::Escape;
        }
        Movement::Stop
    }
}
