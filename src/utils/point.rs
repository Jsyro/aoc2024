use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub struct Point {
    pub y: isize,
    pub x: isize,
}

impl Point {
    pub fn new(y: isize, x: isize) -> Self {
        Self { y, x }
    }

    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    pub fn up() -> Self {
        Self::new(-1, 0)
    }

    pub fn down() -> Self {
        Self::new(1, 0)
    }

    pub fn left() -> Self {
        Self::new(0, -1)
    }

    pub fn right() -> Self {
        Self::new(0, 1)
    }

    pub fn orthogonal() -> impl Iterator<Item = Self> {
        return vec![Self::up(), Self::down(), Self::left(), Self::right()].into_iter();
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.y + rhs.y, self.x + rhs.x)
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.y += rhs.y;
        self.x += rhs.x;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.y - rhs.y, self.x - rhs.x)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.y -= rhs.y;
        self.x -= rhs.x;
    }
}
