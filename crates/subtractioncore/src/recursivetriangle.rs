use crate::matrix::*;
use crate::newutilities::*;
use crate::nimsequence::*;
use std::collections::VecDeque;


/// stores the recursive triangles
/// in a recursive region of the main portion of the space
/// The grandparent nodes are listed first,
/// and then the current nodes,
/// and, finally, the recursive depth
pub struct RecursiveTriangle{pub grandparentpoints: Matrix, pub currentpoints: Matrix, pub recursiondepth: u64, pub matrixtrianglesstring: String}

impl std::fmt::Debug for RecursiveTriangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	let mypoint = self.currentpoints.first_row_as_point()+self.currentpoints.second_row_as_point()+self.currentpoints.third_row_as_point();
	//let mut mypointseq: VecDeque<SequenceNode> = VecDeque::new();
        let mut mypointblockseq: VecDeque<VecDeque<SequenceNode>> = VecDeque::new();
	let mysequence = NimSequence::new(mypoint);
        let mut mypointseq = mysequence.seq;
	let mypointblocklength = mypoint.get_blocklength();
	//for i in 0..mypointseq.len() {
	//    print!("{}^{} ", mypointseq[i].value, mypointseq[i].times_repeated);
	//}
	//println!("");
	
	put_the_full_output_into_blocks (&mut mypointseq, &mut mypointblockseq, mypointblocklength);
	//println!("hope this is 2 {}", mypointblockseq.len());
	//compress_string_all_block_sizes(&mut mypointblockseq[0]);
	//compress_string_all_block_sizes(&mut mypointblockseq[1]);
	let mut mypointstringseq: VecDeque<String> = VecDeque::new();
	//println!("here we go:");
	for i in 0..mypointblockseq[0].len() {
	    mypointstringseq.push_back(format!("{}^{} ", mypointblockseq[0][i].value, mypointblockseq[0][i].times_repeated));
	}
	NimSequence::compress_string_all_block_sizes(&mut mypointstringseq);
	print!("The grandparent point matrix\n{:?}at recursiondepth {:?} and matrixtrianglesstring {} has a point matrix\n{:?}with white point {:?} has nim values\n", self.grandparentpoints, self.recursiondepth, self.matrixtrianglesstring, self.currentpoints, mypoint);
	print!("The first row is ");
	for i in 0..mypointstringseq.len() {
	    print!("{} ", mypointstringseq[i]);
	}
	println!("");
	println!("The second row is {:?}", mypointblockseq[1]);
	
        write!(f, "")
    }
}

impl RecursiveTriangle {
    ///
    pub fn new(mygrandparentpoints: Matrix, mycurrentpoints: Matrix, myrecursiondepth: u64, mymatrixtrianglesstring: String) -> Self {
        RecursiveTriangle{grandparentpoints: mygrandparentpoints, currentpoints: mycurrentpoints, recursiondepth: myrecursiondepth, matrixtrianglesstring: mymatrixtrianglesstring}
    }

    /// decompose the recursive regions
    pub fn decomposerecursivetriange_recursively(self, maximumrecursiondepth: u64, recursivetrianglelist: &mut VecDeque<RecursiveTriangle>) {
        if self.recursiondepth <= maximumrecursiondepth {
	    recursivetrianglelist.push_back(self.clone());

	let recursivetriangle1 = RecursiveTriangle::new(self.grandparentpoints, Matrix::new_from_point(self.currentpoints.first_row_as_point(), self.currentpoints.first_row_as_point()+self.currentpoints.second_row_as_point(), self.currentpoints.first_row_as_point()+self.currentpoints.third_row_as_point()), self.recursiondepth + 1, format!("U{}", self.matrixtrianglesstring));
	recursivetriangle1.decomposerecursivetriange_recursively(maximumrecursiondepth, recursivetrianglelist);
	let recursivetriangle2 = RecursiveTriangle::new(self.grandparentpoints, Matrix::new_from_point(self.currentpoints.first_row_as_point()+self.currentpoints.second_row_as_point(), self.currentpoints.second_row_as_point(), self.currentpoints.second_row_as_point()+self.currentpoints.third_row_as_point()), self.recursiondepth + 1, format!("L{}", self.matrixtrianglesstring));
	recursivetriangle2.decomposerecursivetriange_recursively(maximumrecursiondepth, recursivetrianglelist);
	let recursivetriangle3 = RecursiveTriangle::new(self.grandparentpoints, Matrix::new_from_point(self.currentpoints.first_row_as_point()+self.currentpoints.third_row_as_point(), self.currentpoints.second_row_as_point()+self.currentpoints.third_row_as_point(), self.currentpoints.third_row_as_point()), self.recursiondepth + 1, format!("R{}", self.matrixtrianglesstring));
	recursivetriangle3.decomposerecursivetriange_recursively(maximumrecursiondepth, recursivetrianglelist);

        }
    }


}

//impl Copy for RecursiveTriangle {}

impl Clone for RecursiveTriangle {
    fn clone(&self) -> Self {
        Self {grandparentpoints: self.grandparentpoints, currentpoints: self.currentpoints, recursiondepth: self.recursiondepth, matrixtrianglesstring: format!("U{}", self.matrixtrianglesstring)}
    }
}

