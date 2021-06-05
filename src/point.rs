//! Structures for working with discrete 2d coordinates.
//!
//! Point = (x, y)
//! Rectangle = (x, y, w, h)
//! PointMap = {(x, y)}   // TODO!
//!
//! All variables `x`, `y`, `w` and `h` are i32.
//!

// TODO: PointMap
// TODO: impl Iterator for Point
// TODO: Rectangle.left(), top(), bottom(), right()
// TODO: TraitContains
// TODO: Tests

use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    #[allow(dead_code)]
    pub fn new(x: i32, y: i32) -> Self {
        Point {
            x,
            y
        }
    }

    /// Return a new point with an offset (dx, dy).
    pub fn offset(&self, dx: i32, dy: i32) -> Point {
        Point { x: self.x + dx, y: self.y + dy}
    }
}

impl From<(i32, i32)> for Point {
    fn from(xy: (i32, i32)) -> Self {
        Point {
            x: xy.0,
            y: xy.1
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}


impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y
        };
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}


impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y
        };
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Rectangle {
    pub y1: i32,
    pub x1: i32,
    pub y2: i32, // (x2, y2) is inclusive
    pub x2: i32,
}

impl Rectangle {
    pub fn width(&self) -> i32 {
        self.x2 - self.x1 + 1
    }

    pub fn height(&self) -> i32 {
        self.y2 - self.y1 + 1
    }

    #[allow(dead_code)]
    pub fn size(&self) -> Point {
        Point { x: self.width(), y: self.height() }
    }
    
    pub fn top_left(&self) -> Point {
        Point { x: self.x1, y: self.y1 }
    }

    pub fn top_right(&self) -> Point {
        Point { x: self.x2, y: self.y1 }
    }

    pub fn bottom_left(&self) -> Point {
        Point { x: self.x1, y: self.y2 }
    }

    pub fn bottom_right(&self) -> Point {
        Point { x: self.x2, y: self.y2 }
    }

    pub fn center(&self)-> Point {
        Point {
            x: self.x1 + ((self.x2 - self.x1) / 2) as i32,
            y: self.y1 + ((self.y2 - self.y1) / 2) as i32
        }
    }

    // return true if the given Point p lies within the Rectangle
    pub fn contains(&self, p: &Point) -> bool {
        (p.x >= self.x1)
            && (p.x <= self.x2)
            && (p.y >= self.y1)
            && (p.y <= self.y2)
    }

    // return iterator over all points of the rect
    pub fn iter(&self) -> impl Iterator<Item=Point> {
        RectangleIterator::new(self.clone())
    }
}



impl From<(i32, i32, i32, i32)> for Rectangle {
    fn from(xywh: (i32, i32, i32, i32)) -> Rectangle {
        Rectangle {
            x1: xywh.0,
            y1: xywh.1,
            x2: xywh.0 + xywh.2,
            y2: xywh.1 + xywh.3,
        }
    }
}

impl From<(Point, Point)> for Rectangle {
    fn from(points: (Point, Point)) -> Rectangle {
        // TODO: swap points if p1 < p2 ???
        let p1 = points.0;
        let p2 = points.1;
        Rectangle {
            y1: p1.y,
            x1: p1.x,
            y2: p2.y - p1.y,
            x2: p2.x - p1.x
        }
    }
}


struct RectangleIterator {
    x1: i32, x2: i32, y2: i32,
    point: Point
}


impl RectangleIterator {
    fn new(rect: Rectangle) -> RectangleIterator {
        let x1 = std::cmp::min(rect.x1, rect.x2);
        let x2 = std::cmp::max(rect.x1, rect.x2);
        let y1 = std::cmp::min(rect.y1, rect.y2);
        let y2 = std::cmp::max(rect.y1, rect.y2) ;

        RectangleIterator {
            x1, x2, y2,
            point: Point { x: x1 - 1, y: y1 }
        }
    }
}

// cartesian product of (x1..x2) x (y1..y2)
impl Iterator for RectangleIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.point.x < self.x2 {
            self.point.x += 1; // advance column
            Some(self.point)
        } else {
            // reset column, advance row
            self.point.x = self.x1;
            self.point.y += 1;
            if self.point.y <= self.y2 {
                Some(self.point)
            } else {
                // reached last row and column
                None
            }
        }
    }
}


pub type PointSet = HashSet<Point>;
