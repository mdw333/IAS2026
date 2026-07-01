use crate::newutilities::*;
use crate::nimsequence::*;
use crate::colortype::*;
use crate::line::*;

/// A 3-coordinate point, e.g., an x,y,z triple, of positive values

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {pub x: u64, pub y: u64, pub z: u64}

impl Point {
    pub fn new(x: u64, y: u64, z: u64) -> Self {
        Point { x, y, z }
    }

    /// Extract the three values from the point as a tuple of u64 values
    pub fn as_tuple(&self) -> (u64, u64, u64) {
        (self.x, self.y, self.z)
    }

    /// Calculates the green line that passes through this point.
    pub fn green_line(&self) -> Line {
        Line::new(
            self.y as i64 + self.z as i64,
            -1 * (self.x as i64 + self.z as i64),
            self.y as i64 - self.x as i64,
        )
    }
    
    /// Calculates the orange line that passes through this point.
    pub fn orange_line(&self) -> Line {
        Line::new(
            self.y as i64 + self.z as i64,
            self.z as i64 - self.x as i64,
            -1 * (self.x as i64 + self.y as i64),
        )
    }

    /// Calculates the purple line that passes through this point.
    pub fn purple_line(&self) -> Line {
        Line::new(
            self.z as i64 - self.y as i64,
            self.x as i64 + self.z as i64,
            -1 * (self.x as i64 + self.y as i64),
        )
    }

    /// when analyzing the sequence of all nim values for an x,y,z triple,
    /// and also when printing the output of all the nim values,
    /// it is helping to split the output into blocks;
    /// this function specifies how wide each block should be,
    /// i.e., what length of nim values (altogether, once uncompressed per line)
    /// should be on each line of output.
    /// Notice that the default is to use the period length for the block length,
    /// but with the exceptions that, for green, orange, purple, and white,
    /// we use longer block lengths; using the period by itself is too short!
    pub fn get_blocklength(&self) -> u64 {
        let myperiod = NimSequence::new( *self ).periodlength;
        let mut myblocklength = myperiod;
        if myperiod == gcd(gcd(self.x + self.y, self.x + self.z), self.y + self.z) {
            myblocklength = u64::MAX;
        } else if myperiod == gcd(self.x + self.z, self.y + self.z) {
            // for green
            myblocklength = self.x + self.z; // instead, use the length for blue
        } else if myperiod == gcd(self.x + self.y, self.y + self.z) {
            // for orange
            myblocklength = self.y + self.z; // instead, use the length for yellow
        } else if myperiod == gcd(self.x + self.y, self.x + self.z) {
            // for purple
            myblocklength = self.x + self.y; // instead, use the length for red
        }
        return myblocklength;
    }
    
    /// for an x,y,z, triple with known period length,
    /// return the color associated with the x,y,z triple
    pub fn get_color(&self) -> Color {
        let period_len = NimSequence::new( *self ).periodlength;
        if self.z == self.x + self.y {
            Color::Gray
        } else if period_len == gcd(gcd(self.x + self.y, self.x + self.z), self.y + self.z) {
            Color::White
        } else if period_len == gcd(self.x + self.z, self.y + self.z) {
            Color::Green
        } else if period_len == gcd(self.x + self.y, self.y + self.z) {
            Color::Orange
        } else if period_len == gcd(self.x + self.y, self.x + self.z) {
            Color::Purple
        } else if period_len == self.y + self.z {
            Color::Yellow
        } else if period_len == self.x + self.z {
            Color::Blue
        } else if period_len == self.x + self.y {
            Color::Red
        } else {
            unreachable!()
        }
    }

    
}

/// how to create a new point by adding two points
impl std::ops::Add for Point {
    /// adding two points creates a new point
    type Output = Self;

    /// we perform the addition pointwise
    /// (add the x values, add the y values, add the z values)
    fn add(self, other: Self) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

/// how to create a new point by subtracting two points
impl std::ops::Sub for Point {
    /// subtracting two points creates a new point
    type Output = Self;

    /// we perform the subtraction pointwise
    /// (subtract the x values, subtract the y values, subtract the z values)
    fn sub(self, other: Self) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul<u64> for Point {
    type Output = Self;

    /// create a new point by scaling (i.e., multiplying) pointwise, by a non-negative integer rhs
    fn mul(self, rhs: u64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}


impl std::ops::Div<u64> for Point {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
        // these assert conditions were suggested by ChatGPT
        assert!(rhs != 0, "division by zero");
        assert!(self.x % rhs == 0);
        assert!(self.y % rhs == 0);
        assert!(self.z % rhs == 0);        
        Point::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

