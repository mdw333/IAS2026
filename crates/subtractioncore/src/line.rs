use crate::point::*;

/// The 3 coefficients of a line, e.g., an a,b,c triple, of values for line ax + by + cz = 0

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Line{pub a: i64, pub b: i64, pub c: i64}

impl Line {
    pub fn new(a: i64, b: i64, c: i64) -> Self {
        Line { a, b, c }
    }

    ///impl std::fmt::Debug for Line {
    ///    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    /// Extract the three values from the line as a string
    pub fn as_string(&self) -> String {
        format!("{}x{}{}y{}{}z", self.a, if self.b >= 0 { "+" } else { "" }, self.b, if self.c >= 0 { "+" } else { "" }, self.c)
    }

    pub fn evaluate_at_point(&self, point: &Point) -> i64 {
	let ax = (self.a as i128).checked_mul(point.x as i128).unwrap();
	let by = (self.b as i128).checked_mul(point.y as i128).unwrap();
	let cz = (self.c as i128).checked_mul(point.z as i128).unwrap();
	( ax.checked_add(by).unwrap().checked_add(cz).unwrap() ) as i64
    }

    pub fn dot(&self, other: &Line) -> i64 {
	let ax = (self.a as i128).checked_mul(other.a as i128).unwrap();
	let by = (self.b as i128).checked_mul(other.b as i128).unwrap();
	let cz = (self.c as i128).checked_mul(other.c as i128).unwrap();
	( ax.checked_add(by).unwrap().checked_add(cz).unwrap() ) as i64
    }

}

/// how to create a new line by adding two lines
impl std::ops::Add for Line {
    /// adding two lines creates a new line
    type Output = Self; 

    /// we perform the addition pointwise
    /// (add the a values, add the b values, add the c values)
    fn add(self, other: Self) -> Self::Output {
        Line::new(self.a + other.a, self.b + other.b, self.c + other.c)
    }
}
