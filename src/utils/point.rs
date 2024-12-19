use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub struct Point {
    pub y: i32,
    pub x: i32,
}

impl Point {
    pub fn new(y: i32, x: i32) -> Self {
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
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
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
