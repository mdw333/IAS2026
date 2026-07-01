use crate::point::*;
use crate::region::*;
use crate::regiontype::*;

/// stores the information needed for a hexagon
///pub struct Hexagon{pub whitepoint: Point, pub t: u64, pub r: u64, pub regiontable: RegionTable}
pub struct Hexagon{pub whitepoint: Point, pub regiontable: RegionTable}

impl std::fmt::Debug for Hexagon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The center of the hexagon is: {:?}", self.whitepoint)
    }
}

impl Hexagon {
    ///
    pub fn new(mypoint: Point) -> Self {
//	let myt = mypoint.t();
//	let myr = mypoint.r();
	let myregiontable = RegionTable {
	  data: [Region::new(mypoint,regiontype::sw),
                 Region::new(mypoint,regiontype::ss),
                 Region::new(mypoint,regiontype::se),
                 Region::new(mypoint,regiontype::ne),
                 Region::new(mypoint,regiontype::nn),
                 Region::new(mypoint,regiontype::nw)],
	};
//        Hexagon{whitepoint: mypoint, t: myt, r: myr, regiontable: myregiontable}
        Hexagon{whitepoint: mypoint, regiontable: myregiontable}
    }
}

