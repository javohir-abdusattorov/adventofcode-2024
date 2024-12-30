use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    pub fn up() -> Self {
        Self::new(0, -1)
    }

    pub fn down() -> Self {
        Self::new(0, 1)
    }

    pub fn left() -> Self {
        Self::new(-1, 0)
    }

    pub fn right() -> Self {
        Self::new(1, 0)
    }

    pub fn diagonal() -> impl Iterator<Item = Self> {
        (-1..=1).flat_map(|x| {
            (-1..=1)
                .filter(move |&y| x != 0 && y != 0)
                .map(move |y| Self::new(x, y))
        })
    }

    pub fn moore() -> impl Iterator<Item = Self> {
        (-1..=1).flat_map(|x| {
            (-1..=1)
                .filter(move |&y| x != 0 || y != 0)
                .map(move |y| Self::new(x, y))
        })
    }

    pub fn von_neumann() -> impl Iterator<Item = Self> {
        (-1..=1).flat_map(|x| {
            (-1..=1)
                .filter(move |&y| (x == 0) ^ (y == 0))
                .map(move |y| Self::new(x, y))
        })
    }

    pub fn manhattan_distance(&self, other: &Point) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn opposite(&self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign for Point {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<usize> for Point {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self::new(self.x * rhs as i32, self.y * rhs as i32)
    }
}

impl MulAssign<usize> for Point {
    fn mul_assign(&mut self, rhs: usize) {
        self.x *= rhs as i32;
        self.y *= rhs as i32;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl From<u8> for Point {
    fn from(value: u8) -> Self {
        match value {
            b'^' | b'U' => Self::up(),
            b'v' | b'D' => Self::down(),
            b'<' | b'L' => Self::left(),
            b'>' | b'R' => Self::right(),
            _ => unreachable!(),
        }
    }
}

impl From<Point> for u8 {
    fn from(value: Point) -> Self {
        match value {
            Point { x: 0, y: -1 } => b'^',
            Point { x: 0, y: 1 } => b'v',
            Point { x: -1, y: 0 } => b'<',
            Point { x: 1, y: 0 } => b'>',
            _ => unreachable!(),
        }
    }
}