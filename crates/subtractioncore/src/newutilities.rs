//use eframe::egui;
//use egui::Color32;
use crate::triangle::*;
use crate::matrix::*;
use crate::line::*;
use std::collections::VecDeque;
use crate::nimsequence::*;


/// the greatest common divisor of two integers
/// https://gist.github.com/victor-iyi/8a84185c1d52419b0d4915a648d5e3e1
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

/// the greatest common divisor of three integers
pub fn gcd3(m: u64, n: u64, o: u64) -> u64 {
    gcd(gcd(m, n), o)
}





// given three points (whitepoint, secondpoint, thirdpoint),
// we first calculate three points on the (strict) interior of a triangle
// and then, knowing the number of times that a Nim value has been repeated
// (at a certain stage in the sequence of Nim values) for these three points,
// we can find the constants:
// myfirstreturnvalue,mysecondreturnvalue,mythirdreturnvalue that express
// the exponent (in general) in terms of x-beta-gamma,gamma,beta, like this:
// myfirstreturnvalue*(x-beta-gamma)
//     +mysecondreturnvalue*gamma + mythirdreturnvalue*beta

// we send this function 3 exponents:
// for instance:
// for myfirsttestpoint, we will have a certain number of copies of nim value 2, like this:  2^{19}
// for mysecondtestpoint, we might have:  2^{37}
// and for mythirdtestpoint, we might have:  2^{5}

// now we want to find (for instance)
// the values a, b, c such that
// a*(x-beta-gamma) + b*gamma + c*beta
// gives these three values 19, and 37, and 5
// for myfirsttestpoint, mysecondtestpoint, and mythirdtestpoint, respectively,
// and therefore a*(x-beta-gamma) + b*gamma + c*beta
// is the correct formula in general.

pub fn find_exponents( exponent1: u64, exponent2: u64, exponent3: u64, mytriangle: &Triangle) -> [i64;3] {

    // we could improve on this setup by using matrices!!!
    
    let whitepoint = mytriangle.pointmatrix.first_row_as_point();
    let secondpoint = mytriangle.pointmatrix.second_row_as_point();
    let thirdpoint = mytriangle.pointmatrix.third_row_as_point();
    
    let myfirsttestpoint = whitepoint + whitepoint + secondpoint + thirdpoint;
    let mysecondtestpoint = whitepoint + secondpoint + secondpoint + thirdpoint;
    let mythirdtestpoint = whitepoint + secondpoint + thirdpoint + thirdpoint;

    let a = Matrix::new_from_point(whitepoint, secondpoint, thirdpoint);
    let b = Matrix::new_from_point(myfirsttestpoint, mysecondtestpoint, mythirdtestpoint).inverse_without_determinant();
    let bdet = Matrix::new_from_point(myfirsttestpoint, mysecondtestpoint, mythirdtestpoint).determinant();

    let myline = (a*b).multiply_on_right_by_columnvec(Line::new(exponent1 as i64,exponent2 as i64,exponent3 as i64));
    
    if (myline.a % bdet != 0) || (myline.b % bdet != 0) || (myline.c % bdet != 0) {
        println!("warning about divisibility");
    }

    let myfirstreturnvalue = myline.a/bdet;
    let mysecondreturnvalue = myline.b/bdet;
    let mythirdreturnvalue = myline.c/bdet;
        
    return [myfirstreturnvalue,mysecondreturnvalue,mythirdreturnvalue]
}


pub fn put_the_full_output_into_blocks (seq: &mut VecDeque<SequenceNode>, blockseq: &mut VecDeque<VecDeque<SequenceNode>>, myblocklength: u64) {

    blockseq.push_back(VecDeque::new());

    let mut myrowcounter = 0;

    for i in 0..seq.len() {
        if seq[i].value == u8::MAX {
            if i != 0 {
                blockseq.push_back(VecDeque::new());
                myrowcounter = 0;
            }
        }
        else {
            if myrowcounter == myblocklength {
                blockseq.push_back(VecDeque::new());
                    myrowcounter = 0;
                }
            myrowcounter += seq[i].times_repeated;
            if myrowcounter <= myblocklength {
                blockseq.back_mut().unwrap().push_back(SequenceNode::new(seq[i].value, seq[i].times_repeated));
            }
            else {
                blockseq.back_mut().unwrap().push_back(SequenceNode::new(seq[i].value, seq[i].times_repeated - (myrowcounter - myblocklength)));
                blockseq.push_back(VecDeque::new());
                blockseq.back_mut().unwrap().push_back(SequenceNode::new(seq[i].value, myrowcounter - myblocklength));
                myrowcounter = myrowcounter - myblocklength;
            }
        }
    }
}



