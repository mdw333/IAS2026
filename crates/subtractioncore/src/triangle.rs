use crate::matrix::*;
use crate::point::*;
use crate::regiontype::*;
use std::collections::VecDeque;


/// stores the corners, lines, and region of a triangle in a hexagon
#[allow(non_snake_case)]
pub struct Triangle {
    pub pointmatrix: Matrix,
    pub linematrix: Matrix,
    pub region: regiontype,
}
// because we write the points in the order:
// whitepoint, secondpoint, thirdpoint,
// it makes the most sense to write the lines in the order:
// bottomline, rightline, leftline

impl std::fmt::Debug for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?} {:?}",
            self.pointmatrix,
            self.linematrix,
            self.region
        )
    }
}

impl Triangle {
    #[allow(non_snake_case)]
    pub fn new(
        mut mypointmatrix: Matrix,
        myregion: regiontype,
    ) -> Self {
        let mylinematrix = mypointmatrix.inverse_without_determinant().divide_by_integer(mypointmatrix.determinant());
        Triangle {
            pointmatrix: mypointmatrix,
            linematrix: mylinematrix,
            region: myregion,
        }
    }

    /// suggested by ChatGPT
    fn opposite_signs(a: i64, b: i64) -> bool {
        (a < 0 && b > 0) || (a > 0 && b < 0)
    }

    #[allow(non_snake_case)]
    pub fn decompose_triangle_recursively(trianglepoints: Matrix,  myregiontype: regiontype, mytrianglelist: &mut VecDeque<Triangle>) {
        // assume that the first row of the Matrix is a white point
        let whitepoint = trianglepoints.first_row_as_point();
        let secondpoint = trianglepoints.second_row_as_point();
        let thirdpoint = trianglepoints.third_row_as_point();
        if Triangle::opposite_signs(whitepoint.orange_line().evaluate_at_point(&secondpoint),
                      whitepoint.orange_line().evaluate_at_point(&thirdpoint)) ||
            Triangle::opposite_signs(whitepoint.purple_line().evaluate_at_point(&secondpoint),
                      whitepoint.purple_line().evaluate_at_point(&thirdpoint)) ||
            Triangle::opposite_signs(whitepoint.green_line().evaluate_at_point(&secondpoint),
                      whitepoint.green_line().evaluate_at_point(&thirdpoint)) {
            Triangle::decompose_triangle_recursively( Matrix::new_from_point(
               Point::new(1,0,0),Point::new(0,1,0),Point::new(0,1,1)
               )*trianglepoints, myregiontype, mytrianglelist);
	    Triangle::decompose_triangle_recursively( Matrix::new_from_point(
               Point::new(1,0,0),Point::new(0,1,1),Point::new(0,0,1)
               )*trianglepoints, myregiontype, mytrianglelist);
        }
        else {
    	    mytrianglelist.push_back(Triangle::new(trianglepoints, myregiontype));
        }
    }



}
