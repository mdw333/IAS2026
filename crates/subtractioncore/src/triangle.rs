use crate::matrix::*;
use crate::point::*;
use crate::regiontype::*;
use std::collections::VecDeque;
use crate::colortype::*;
use crate::nimsequence::*;
use crate::newutilities::*;


/// stores the corners, lines, and region of a triangle in a hexagon
#[allow(non_snake_case)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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

    /// predicts the color of a triangle bounded by the
    /// three points called whitepoint, secondpoint, thirdpoint
    pub fn predictedcolorfunction(&self) -> Color {
        let mymidpoint = self.pointmatrix.second_row_as_point() + self.pointmatrix.third_row_as_point();
        let tempgreen = self.pointmatrix.first_row_as_point().green_line().evaluate_at_point(&mymidpoint);
        let temporange = self.pointmatrix.first_row_as_point().orange_line().evaluate_at_point(&mymidpoint);
        let temppurple = self.pointmatrix.first_row_as_point().purple_line().evaluate_at_point(&mymidpoint);

        if ((tempgreen < 0) && (temporange > 0) && (temppurple > 0))
            || ((tempgreen > 0) && (temporange < 0) && (temppurple < 0))
        {
            Color::Yellow
        } else if ((tempgreen < 0) && (temporange < 0) && (temppurple > 0))
            || ((tempgreen > 0) && (temporange > 0) && (temppurple < 0))
        {
            Color::Red
        } else if ((tempgreen < 0) && (temporange < 0) && (temppurple < 0))
            || ((tempgreen > 0) && (temporange > 0) && (temppurple > 0))
        {
            Color::Blue
        } else {
            panic!("FIXME")     // this could perhaps be fixed with the assert idea
        }
    }

    #[allow(non_snake_case)]
    /// print the output in blocks
    pub fn calculatetriangle(&self, lastlineonly: bool, returnstring: &mut String) {
        //    *returnstring += &String::from(format!("{:?}  {:?}  {:?} color {:?} gives ", self.whitepoint, self.secondpoint, self.thirdpoint, predictedcolorfunction(self.whitepoint, self.secondpoint, self.thirdpoint)));
        // the points we should test are:
        // 2*whitepoint + secondpoint + thirdpoint
        // whitepoint + 2*secondpoint + thirdpoint
        // whitepoint + secondpoint + 2*thirdpoint
        let myfirsttestpoint =
            self.pointmatrix.first_row_as_point() + self.pointmatrix.first_row_as_point() + self.pointmatrix.second_row_as_point() + self.pointmatrix.third_row_as_point();
        let mysecondtestpoint =
            self.pointmatrix.first_row_as_point() + self.pointmatrix.second_row_as_point() + self.pointmatrix.second_row_as_point() + self.pointmatrix.third_row_as_point();
        let mythirdtestpoint =
            self.pointmatrix.first_row_as_point() + self.pointmatrix.second_row_as_point() + self.pointmatrix.third_row_as_point() + self.pointmatrix.third_row_as_point();

        let mut firstseq: VecDeque<SequenceNode> = VecDeque::new();
        let mut firstblockseq: VecDeque<VecDeque<SequenceNode>> = VecDeque::new();

        let mut secondseq: VecDeque<SequenceNode> = VecDeque::new();
        let mut secondblockseq: VecDeque<VecDeque<SequenceNode>> = VecDeque::new();

        let mut thirdseq: VecDeque<SequenceNode> = VecDeque::new();
        let mut thirdblockseq: VecDeque<VecDeque<SequenceNode>> = VecDeque::new();

        let mycode = 2;

        //let firstmyperiod = NimSequence::new(myfirsttestpoint).seq;
        //let secondmyperiod = NimSequence::new(mysecondtestpoint).seq;
        //let thirdmyperiod = NimSequence::new(mythirdtestpoint).seq;
        let myfirstblocklength = myfirsttestpoint.get_blocklength();
        let mysecondblocklength = mysecondtestpoint.get_blocklength();
        let mythirdblocklength = mythirdtestpoint.get_blocklength();

        // finish putting the full output into seq
        // this means that we will:
        // remove the excess sequence length
        // and will also remove the buffer node at the front

        // I THINK IT IS SAFE TO REMOVE THESE THREE LINES OF CODE
        // BECAUSE generate_sequence NOW HAS removing_excess_seq_length AT THE END OF ITS CODE
        // BUT I ONLY DID THIS ON 14 MARCH 2026 SO WE WANT TO BE CAREFUL ABOUT THIS.
        //removing_excess_seq_length (&mut firstseq, firstmyperiod);
        //removing_excess_seq_length (&mut secondseq, secondmyperiod);
        //removing_excess_seq_length (&mut thirdseq, thirdmyperiod);

        if mycode == 1 {
            NimSequence::print_triplecheck_format(myfirsttestpoint, '^');
            NimSequence::print_triplecheck_format(mysecondtestpoint, '^');
            NimSequence::print_triplecheck_format(mythirdtestpoint, '^');
        } else if mycode == 2 {
            put_the_full_output_into_blocks(&mut firstseq, &mut firstblockseq, myfirstblocklength);
            put_the_full_output_into_blocks(
                &mut secondseq,
                &mut secondblockseq,
                mysecondblocklength,
            );
            put_the_full_output_into_blocks(&mut thirdseq, &mut thirdblockseq, mythirdblocklength);
        }
        if (firstblockseq.len() != secondblockseq.len())
            || (firstblockseq.len() != thirdblockseq.len())
        {
            println!("the number of rows in the blocks do not match");
        }

        for a in 0..firstblockseq.len() {
            if (firstblockseq[a].len() != secondblockseq[a].len())
                || (firstblockseq[a].len() != thirdblockseq[a].len())
            {
                println!("the number of parts of row {} do not match", a);
            }
        }
        for a in 0..firstblockseq.len() {
            let mut stringseq: VecDeque<String> = VecDeque::new();
            if (!lastlineonly) || (a == firstblockseq.len() - 1) {
                for b in 0..firstblockseq[a].len() {
                    let [
                        calculatedexponent1,
                        calculatedexponent2,
                        calculatedexponent3,
                    ] = find_exponents(
                        firstblockseq[a][b].times_repeated,
                        secondblockseq[a][b].times_repeated,
                        thirdblockseq[a][b].times_repeated,
                        &self,
                    );
                    let mystring = format!(
                        "{}^{{[{},{},{}]}} ",
                        firstblockseq[a][b].value,
                        calculatedexponent1,
                        calculatedexponent2,
                        calculatedexponent3
                    );
                    stringseq.push_back(mystring);
                }
            }

            NimSequence::compress_string_all_block_sizes(&mut stringseq);

            // debugging
            if (!lastlineonly) || (a == firstblockseq.len() - 1) {
                //println!("We got this row for our good output:");
                for i in 0..stringseq.len() {
                    *returnstring += &stringseq[i];
                }
                *returnstring += "\n";
                //println!("{:?}", stringseq);
            }
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
