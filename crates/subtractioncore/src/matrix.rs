use crate::point::*;
use crate::line::*;


/// stores a 3-by-3 matrix of real values (positive or negative)
/// a11,a12,a13
/// a21,a22,a23
/// a31,a32,a33
/// of a matrix
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Matrix{pub a11: i64, pub a12: i64, pub a13: i64, pub a21: i64, pub a22: i64, pub a23: i64, pub a31: i64, pub a32: i64, pub a33: i64}


impl Matrix {

    /// creates a new 3-by-3 matrix, with col1, col2, col3 as the columns
    /// reminder:  we are tending to use column vectors that are integers,
    /// either positive or negative
    pub fn new_from_columnvec(col1: Line, col2: Line, col3: Line) -> Self {
        Matrix { a11: col1.a, a12: col2.a, a13: col3.a, a21: col1.b, a22: col2.b, a23: col3.b, a31: col1.c, a32: col2.c, a33: col3.c }
    }

    /// creates a new 3-by-3 matrix, with point1, point2, point3 as the rows
    /// reminder:  we are tending to use points (as the matrix rows)
    /// that are positive integers, so we are converting from u64 to i64
    pub fn new_from_point(point1: Point, point2: Point, point3: Point) -> Self {
        Matrix { a11: point1.x as i64, a12: point1.y as i64, a13: point1.z as i64, a21: point2.x as i64, a22: point2.y as i64, a23: point2.z as i64, a31: point3.x as i64, a32: point3.y as i64, a33: point3.z as i64 }
    }

    /// calculates the determinant of a 3-by-3 matrix
    pub fn determinant(&mut self) -> i64 {
	let b11 = self.a11 as i128;
	let b12 = self.a12 as i128;
	let b13 = self.a13 as i128;
	let b21 = self.a21 as i128;
	let b22 = self.a22 as i128;
	let b23 = self.a23 as i128;
	let b31 = self.a31 as i128;
	let b32 = self.a32 as i128;
	let b33 = self.a33 as i128;
        (b11*(b22*b33 - b23*b32) - b12*(b21*b33 - b23*b31) + b13*(b21*b32 - b22*b31)) as i64  // check with Alexander about how to make sure that we are safe to convert back to i64
    }

    /// calculates the inverse of a 3-by-3 matrix
    /// BUT does not divide by the determinant;
    /// we still need to divide by the determinant as a separate operation
    /// Motivation:  If we divide by the determinant, then sometimes
    /// we get real numbers (no longer integers),
    /// but once we multiply the matrix inverse by other things,
    /// the numbers get bigger
    /// and we will (often? usually? always in our cases?)
    /// be able to (afterwards) divide by the determinant
    /// and still have integers after the division by the determinant.
    pub fn inverse_without_determinant(&mut self) -> Matrix {
	let b11 = self.a11 as i128;
	let b12 = self.a12 as i128;
	let b13 = self.a13 as i128;
	let b21 = self.a21 as i128;
	let b22 = self.a22 as i128;
	let b23 = self.a23 as i128;
	let b31 = self.a31 as i128;
	let b32 = self.a32 as i128;
	let b33 = self.a33 as i128;
        Matrix::new_from_columnvec(
	  Line::new((b22*b33 - b23*b32) as i64,
		((-1 as i128)*(b21*b33 - b23*b31)) as i64,
		(b21*b32 - b22*b31) as i64),
	  Line::new(((-1 as i128)*(b12*b33 - b13*b32)) as i64,
		(b11*b33 - b13*b31) as i64,
		((-1 as i128)*(b11*b32 - b12*b31)) as i64),
	  Line::new((b12*b23 - b13*b22) as i64,
		((-1 as i128)*(b11*b23 - b13*b21)) as i64,
		(b11*b22 - b12*b21) as i64))
    }

    pub fn divide_by_integer(&mut self, mydivisor: i64) -> Matrix {
	let mut myflag = true;
	if self.first_row_as_line().a % mydivisor != 0 { myflag = false; }
	if self.first_row_as_line().b % mydivisor != 0 { myflag = false; }
	if self.first_row_as_line().c % mydivisor != 0 { myflag = false; }
	if self.second_row_as_line().a % mydivisor != 0 { myflag = false; }
	if self.second_row_as_line().b % mydivisor != 0 { myflag = false; }
	if self.second_row_as_line().c % mydivisor != 0 { myflag = false; }
	if self.third_row_as_line().a % mydivisor != 0 { myflag = false; }
	if self.third_row_as_line().b % mydivisor != 0 { myflag = false; }
	if self.third_row_as_line().c % mydivisor != 0 { myflag = false; }
	if myflag == false {
	    println!("we have a matrix determinant error!");
	}
        Matrix::new_from_columnvec(
	    Line::new(self.a11/mydivisor,
		      self.a21/mydivisor,
		      self.a31/mydivisor),
	    Line::new(self.a12/mydivisor,
		      self.a22/mydivisor,
		      self.a32/mydivisor),
	    Line::new(self.a13/mydivisor,
		      self.a23/mydivisor,
		      self.a33/mydivisor))
    }

    pub fn multiply_on_right_by_columnvec(&mut self, mycolumnvec: Line) -> Line {
        Line::new(self.first_row_as_line().dot(&mycolumnvec),
	    self.second_row_as_line().dot(&mycolumnvec),
	    self.third_row_as_line().dot(&mycolumnvec))
    }

    pub fn multiply_on_left_by_point(&mut self, mypoint: Point) -> Point {
	let myline = Line::new(mypoint.x as i64, mypoint.y as i64, mypoint.z as i64);
        Point::new(self.first_column().dot(&myline) as u64,
	    self.second_column().dot(&myline) as u64,
	    self.third_column().dot(&myline) as u64)
    }
    
    pub fn first_row_as_line(&self) -> Line {
        Line { a: self.a11, b: self.a12, c: self.a13 }
    }
    
    pub fn first_row_as_point(&self) -> Point {
        Point { x: self.a11 as u64, y: self.a12 as u64, z: self.a13 as u64 }
    }
    
    pub fn second_row_as_line(&self) -> Line {
        Line { a: self.a21, b: self.a22, c: self.a23 }
    }

    pub fn second_row_as_point(&self) -> Point {
        Point { x: self.a21 as u64, y: self.a22 as u64, z: self.a23 as u64 }
    }

    pub fn third_row_as_line(&self) -> Line {
        Line { a: self.a31, b: self.a32, c: self.a33 }
    }
    
    pub fn third_row_as_point(&self) -> Point {
        Point { x: self.a31 as u64, y: self.a32 as u64, z: self.a33 as u64 }
    }

    pub fn first_column(&self) -> Line {
        Line { a: self.a11, b: self.a21, c: self.a31 }
    }
    
    pub fn second_column(&self) -> Line {
        Line { a: self.a12, b: self.a22, c: self.a32 }
    }
    
    pub fn third_column(&self) -> Line {
        Line { a: self.a13, b: self.a23, c: self.a33 }
    }

    /// prints the rows of a 3-by-3 matrix
    ///impl std::fmt::Debug for Matrix {
    ///    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    pub fn as_string(&self) -> String {
        format!("{} {} {}\n{} {} {}\n{} {} {}\n", self.a11, self.a12, self.a13, self.a21, self.a22, self.a23, self.a31, self.a32, self.a33)
    }
    
}

/// how to create a new matrix by multiplying two matrices
impl std::ops::Mul for Matrix {
    /// multiplying two matrices creates a new matrix
    type Output = Self; 

    fn mul(self, other: Self) -> Self::Output {
        Matrix::new_from_columnvec(
	    Line::new(self.first_row_as_line().dot(&other.first_column()),
		      self.second_row_as_line().dot(&other.first_column()),
		      self.third_row_as_line().dot(&other.first_column())),
	    Line::new(self.first_row_as_line().dot(&other.second_column()),
		      self.second_row_as_line().dot(&other.second_column()),
		      self.third_row_as_line().dot(&other.second_column())),
	    Line::new(self.first_row_as_line().dot(&other.third_column()),
		      self.second_row_as_line().dot(&other.third_column()),
		      self.third_row_as_line().dot(&other.third_column())))
    }
}


