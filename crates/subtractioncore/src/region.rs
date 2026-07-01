use crate::point::*;
use crate::regiontype::*;
use crate::triangle::*;
use crate::matrix::*;
use std::ops::Index;
use std::collections::VecDeque;



// The concept of RegionTable came from Sarah at ChatGPT
// Laura says:  She's everywhere.  She's in the cloud.
pub struct RegionTable {
    pub data: [Region; 6],
}

impl Index<regiontype> for RegionTable {
    type Output = Region;

    fn index(&self, myregion: regiontype) -> &Self::Output {
        &self.data[myregion as usize]
    }
}


/// stores the information needed for a region
pub struct Region{pub pointmatrix: Matrix, pub linematrix: Matrix, region: regiontype, pub trianglelist: VecDeque<Triangle> }

impl std::fmt::Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The point matrix for the region is: {:?} and the line matrix for the region is: {:?} and the region type is {:?}", self.pointmatrix, self.linematrix, self.region)
    }
}

impl Region {
  ///
  pub fn new(mywhitepoint: Point, myregiontype: regiontype) -> Self {

    let myt = ((mywhitepoint.z as f64 - mywhitepoint.y as f64) / (mywhitepoint.x as f64 + mywhitepoint.y as f64)) as u64;
//    let myr = ((mywhitepoint.y as f64) / (mywhitepoint.x as f64)) as u64;
    let mysecondpoint = match myregiontype {
      regiontype::sw => mywhitepoint - Point::new(0,0,1),
      regiontype::ss => mywhitepoint + Point::new(0,1,myt+1),
      regiontype::se => mywhitepoint + Point::new(0,1,myt+2),
      regiontype::ne => mywhitepoint + Point::new(0,0,1),
      regiontype::nn => mywhitepoint - Point::new(0,1,myt+1),
      regiontype::nw => mywhitepoint - Point::new(0,1,myt+2),
    };
    let mythirdpoint = match myregiontype {
      regiontype::sw => mywhitepoint + Point::new(0,1,myt+1),
      regiontype::ss => mywhitepoint + Point::new(0,1,myt+2),
      regiontype::se => mywhitepoint + Point::new(0,0,1),
      regiontype::ne => mywhitepoint - Point::new(0,1,myt+1),
      regiontype::nn => mywhitepoint - Point::new(0,1,myt+2),
      regiontype::nw => mywhitepoint - Point::new(0,0,1),
    };
    let mydeterminant = Matrix::new_from_point(mywhitepoint, mysecondpoint, mythirdpoint).determinant();
    let mut mytempmatrix = Matrix::new_from_point(mywhitepoint, mysecondpoint, mythirdpoint).inverse_without_determinant();
    let myreturnmatrix = mytempmatrix.divide_by_integer(mydeterminant);

    // Now we need to go build mytrianglelist.
    // We decompose a hexagon into all of its triangles
    // using the Stern Brocot concept of splitting,
    // assuming that the hexagon is in the global portion of the space,
    // and away from the transitions between the regions of the space
    let mut mytrianglelist: VecDeque<Triangle> = VecDeque::new();
    //let tempnum = (mywhitepoint.z % (mywhitepoint.x + mywhitepoint.y) ) - mywhitepoint.x;
    //let tempden = mywhitepoint.x + mywhitepoint.y;

    let trianglepoints = Matrix::new_from_point( mywhitepoint,mysecondpoint,mythirdpoint );
    match myregiontype {
        regiontype::sw | regiontype::ne => {mytrianglelist.push_back(Triangle::new(Matrix::new_from_point(
               Point::new(1,0,0),Point::new(0,1,0),Point::new(0,1,1)
               )*trianglepoints, myregiontype));
                                            mytrianglelist.push_back(Triangle::new(Matrix::new_from_point(
               Point::new(1,0,0),Point::new(0,1,1),Point::new(0,0,1)
               )*trianglepoints, myregiontype));},
	regiontype::ss | regiontype::se | regiontype::nn | regiontype::nw => {Triangle::decompose_triangle_recursively(Matrix::new_from_point(mywhitepoint, mysecondpoint, mythirdpoint), myregiontype, &mut mytrianglelist);},
    }

    Region{pointmatrix: Matrix::new_from_point(mywhitepoint, mysecondpoint, mythirdpoint), linematrix: myreturnmatrix, region: myregiontype, trianglelist: mytrianglelist}
  }
}
