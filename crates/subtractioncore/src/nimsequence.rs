use std::cmp::min;
use std::collections::VecDeque;
use crate::point::*;

/// A SequenceNode stores a nim value (0, 1, 2, or 3)
/// and a number of times that the nim value is repeated (i.e., the exponent)

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SequenceNode {
    pub value: u8,
    pub times_repeated: u64,
}

/// constructor for SequenceNode
/// this is how to make a new SequenceNode     SequenceNode::new(3, 19);
impl SequenceNode {
    pub fn new(value: u8, times_repeated: u64) -> Self {
        Self { value, times_repeated }
    }
}

/// allows us to print the contents of a SequenceNode,
/// which is a nim value and a number of times repeated (i.e., an exponent)
impl std::fmt::Debug for SequenceNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}^{}", self.value, self.times_repeated)
    }
}


/// An SequenceCursor is an index into the sequence, offset by a certain amount.
/// For instance, if we point into the history of nim values
/// and we get a SequenceNode 3^19
/// then we might have (for example) already processed 14 of the 19 values
/// so the "lookbehind" is 14, and the "offset" is 5, in this example

pub struct SequenceCursor {
    pub idx: usize,
    pub offset: u64,
}

/// constructor for SequenceCursor
impl SequenceCursor {
    pub fn new(max_len: u64, lookbehind: u64) -> Self {
        Self {
            offset: max_len - lookbehind, idx: 0,
        }
    }

    /// Update this `SequenceCursor` with a new offset into the sequence.
    pub fn update(&mut self, seq: &VecDeque<SequenceNode>, offset: u64) {
        self.offset += offset;

        while self.offset >= seq[self.idx].times_repeated {
            self.offset -= seq[self.idx].times_repeated;
            self.idx += 1;
        }
    }
}



/// A NimSequence stores a sequence of Nim values and the number of times that each nim value is repeated (i.e., the exponent)

//#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NimSequence {pub seq: VecDeque<SequenceNode>, pub preperiodlength: u64, pub periodlength: u64}

impl NimSequence {

    /// compress a string, using a given block size;
    /// we return a boolean value that confirms whether or not a block was able
    /// to be compressed (true is a block was compressed, and false otherwise)
    pub fn compress_string_specific_block_size(stringseq: &mut VecDeque<String>, blocksize: u64) -> bool {
        let mut returnvalue = false;
        let mut keepsearching = 1;
        let mut texvalues0 = 0;
        let mut texvalues1: usize;
        let mut texvalues2: usize;

        while keepsearching == 1 {
            texvalues1 = texvalues0;
            texvalues2 = texvalues0;
        
            if texvalues2 == stringseq.len() {
                keepsearching = 0;
            }

            let mut k = 0;
            while (k < blocksize)&&(keepsearching==1) {
                texvalues2 += 1;
                if texvalues2 == stringseq.len() {
                    keepsearching = 0;
                }
                k += 1;
            }
            if keepsearching == 1 {
                let mut agreementcounter = 0;
                while (texvalues2 < stringseq.len()) && (stringseq[texvalues1] == stringseq[texvalues2]) {
                    agreementcounter += 1;
                    texvalues1 += 1;
                    texvalues2 += 1;
                }
                let numberofblocksmatched = ((agreementcounter as f64) / (blocksize as f64)).floor() as u64;
                if numberofblocksmatched >= 1 {
                    returnvalue = true;
                    texvalues1 = texvalues0;
                    texvalues2 = texvalues0;
            
                    let mut mynewstring = String::from("(");
                    for _ in 0..blocksize {
                        mynewstring = format!("{}{}", mynewstring, stringseq[texvalues1]);
                        texvalues1 += 1;
                    }
                    mynewstring = format!("{}{}{}{}", mynewstring, ")^{", numberofblocksmatched+1, "}");
            
                    texvalues1 = texvalues0;
                    stringseq[texvalues1] = mynewstring;

                    texvalues2 += (numberofblocksmatched as usize + 1)*blocksize as usize;

                    // it is safe to delete everything between texvalues1 and texvalues2
                    let mut tempstringseq: VecDeque<String> = VecDeque::new();
                    for m in 0..texvalues1+1 {
                        let tempstring = stringseq[m].clone();
                        tempstringseq.push_back(tempstring);
                    }
                    for m in texvalues2..stringseq.len() {
                        let tempstring = stringseq[m].clone();
                        tempstringseq.push_back(tempstring);
                    }
                    while stringseq.len() > 0 {
                        stringseq.pop_front();
                    }
                    while tempstringseq.len() > 0 {
                        if let Some(littlenode) = tempstringseq.pop_front() {
                            stringseq.push_back(littlenode);
                        }
                    }
                    texvalues2 -= texvalues2 - texvalues1 - 1;
                    //texvalues1 = texvalues2;
                    texvalues0 = texvalues2;
                }
                else {
                    if texvalues0 < stringseq.len() {
                        texvalues0 += 1;
                    }
                }
            }
        }
        returnvalue
    }

    /// compress a string across all potential block sizes
    pub fn compress_string_all_block_sizes(stringseq: &mut VecDeque<String>) {
        let mut blocksizereached = 0;
        let maxblocksize = 500;
        let mut blocksize = 2;
        while blocksize <= maxblocksize {
            let myresults = NimSequence::compress_string_specific_block_size(stringseq, blocksize);
            if myresults == true {
                if blocksize > blocksizereached {
                    blocksizereached = blocksize;
                }
                blocksize = 2;
            }
            else {
                blocksize += 1;
            }
        }
    }


    /// convert a sequence of SeqNode values to String values
    pub fn to_strings (&self, mychar: char) -> VecDeque<String> {
        let mut mystrings = VecDeque::new();
        for i in 0..self.seq.len() {
            mystrings.push_back(format!("{}{}{} ", self.seq[i].value, mychar, self.seq[i].times_repeated));
        }
        return mystrings;
    }
    
    /// after putting the Nim values into the sequence,
    /// we remove remove the Nim values that go beyond the periodic part
    /// and we also remove the buffer (dummy) node from the front of the sequence
    fn removing_excess_seq_length (mysequence: &mut VecDeque<SequenceNode>, myperiod: u64) {
    
        let mut tempseq: VecDeque<SequenceNode> = VecDeque::new();
        let mut tempcounter = 0;

        // temporarily remove the periodic portion
        while tempcounter < myperiod {
            if let Some(tempseqnode) = mysequence.pop_back() {
                tempcounter += tempseqnode.times_repeated;
                tempseq.push_front(tempseqnode);
            }
        }

        // now append the periodic portion again
        if (tempcounter > myperiod) && (tempseq.front().unwrap().value != tempseq.back().unwrap().value) {
            mysequence.push_back(SequenceNode::new(tempseq.front().unwrap().value, tempcounter - myperiod));
            mysequence.push_back(SequenceNode::new(u8::MAX, 0));
            mysequence.push_back(SequenceNode::new(tempseq.front().unwrap().value, tempseq.front().unwrap().times_repeated - (tempcounter - myperiod)));
            tempcounter -= tempseq.front().unwrap().times_repeated;
            tempseq.pop_front();
        }
        else if (tempcounter > myperiod) && (tempseq.front().unwrap().value == tempseq.back().unwrap().value) {
           let l = tempcounter - myperiod;
           let r = tempseq.front().unwrap().times_repeated - l;
           let b = tempseq.back().unwrap().times_repeated;
           if l < b {
                // if l < b then l(r      b) becomes (l+r     b-l)
                mysequence.push_back(SequenceNode::new(u8::MAX, 0));
                if let Some(tempseqnode) = tempseq.pop_front() {
                    tempcounter -= tempseqnode.times_repeated;
                    mysequence.push_back(tempseqnode);
                }
                tempseq.back_mut().unwrap().times_repeated -= l;
            }
            else if l == b {
                // if l = b then l(r      b) becomes (l+r      x)
                mysequence.push_back(SequenceNode::new(u8::MAX, 0));
                if let Some(tempseqnode) = tempseq.pop_front() {
                    tempcounter -= tempseqnode.times_repeated;
                    mysequence.push_back(tempseqnode);
                }
                tempcounter -= tempseq.back().unwrap().times_repeated;
                tempseq.pop_back();
            }
            else if l > b {
                // if l > b then l(r      b) becomes l-b(r+b      x)
                mysequence.push_back(SequenceNode::new(tempseq.front().unwrap().value, l-b));
                mysequence.push_back(SequenceNode::new(u8::MAX, 0));
                mysequence.push_back(SequenceNode::new(tempseq.front().unwrap().value, r+b));
                tempcounter -= tempseq.front().unwrap().times_repeated;
                tempseq.pop_front();
                tempcounter -= tempseq.back().unwrap().times_repeated;
                tempseq.pop_back();
            }
        }
        else if tempcounter == myperiod {
            if mysequence.back().unwrap().value == tempseq.back().unwrap().value {
                if mysequence.back().unwrap().times_repeated < tempseq.back().unwrap().times_repeated {
                    if let Some(tempseqnode) = mysequence.pop_back() {
                        tempcounter -= tempseqnode.times_repeated;
                        tempseq.back_mut().unwrap().times_repeated -= tempseqnode.times_repeated;
                        mysequence.push_back(SequenceNode::new(u8::MAX, 0));
                        mysequence.push_back(tempseqnode);
                    }
                }
                else if mysequence.back().unwrap().times_repeated == tempseq.back().unwrap().times_repeated {
                    println!("warning");
                }
                else if mysequence.back().unwrap().times_repeated > tempseq.back().unwrap().times_repeated {
                    if let Some(tempseqnode) = mysequence.pop_back() {
                        mysequence.push_back(SequenceNode::new(tempseqnode.value, tempseqnode.times_repeated - tempseq.back().unwrap().times_repeated));
                        mysequence.push_back(SequenceNode::new(u8::MAX, 0));
                        mysequence.push_back(SequenceNode::new(tempseqnode.value, tempseq.back().unwrap().times_repeated));
                        tempcounter -= tempseq.back().unwrap().times_repeated;
                        tempseq.pop_back();
                    }
                }
            }
            else {
                mysequence.push_back(SequenceNode::new(u8::MAX, 0));
            }
        }
        while tempcounter > 0 {
            if let Some(tempseqnode) = tempseq.pop_front() {
                tempcounter -= tempseqnode.times_repeated;
                mysequence.push_back(tempseqnode);
            }
        }
        mysequence.pop_front();    // finally, pop the front (placeholder buffer) node
    }


    /// Compute the `value` and `times_repeated` of the next element(s) in the sequence.
    fn add_new_element(mysequence: &mut VecDeque<SequenceNode>, x_cur: &mut SequenceCursor, y_cur: &mut SequenceCursor, z_cur: &mut SequenceCursor, mex: u8, new_length_generated: &mut u64, current_len: &mut u64) {
        let agreement = min(min(mysequence[x_cur.idx].times_repeated-x_cur.offset, mysequence[y_cur.idx].times_repeated-y_cur.offset), mysequence[z_cur.idx].times_repeated-z_cur.offset);

        // If this `value` is the same as the previous, do not create a new node.
        if mysequence.back().unwrap().value == mex {
            mysequence.back_mut().unwrap().times_repeated += agreement;
        } else {
            mysequence.push_back(SequenceNode::new(mex, agreement));
        }

        // Update cursor offsets:
        x_cur.update(mysequence, agreement);
        y_cur.update(mysequence, agreement);
        z_cur.update(mysequence, agreement);

        *new_length_generated += agreement;

        *current_len += agreement;
    }

    
    /// the mex value (minimally excluded value) of the points in the array
    fn mexfunction(myarray: &[u8]) -> u8 {
        let mut tempmexvalue = 0;
        loop {
            if myarray.contains(&tempmexvalue) {
                tempmexvalue += 1;
            }
            else {
                return tempmexvalue
            }
        }
    }

    
    /// Approximates the maximum period length of NIM values for this point.
    fn max_period_len(x: u64, y: u64, z: u64) -> u64 {
        let mut max_len = y + z;

        if x + y == z {
            if (y - x) % (2 * x) <= x {
                max_len = 2 * z;
            } else {
                max_len *= x;
            }
        }
        max_len *= 2;

        (max_len + z) as u64
    }

    /// Knuth Morris Pratt (KMP) algorithm
    /// finds longest overlaps (i.e., the shortest repeating periods) in a sequence
    fn kmp_preprocess_defined_start(seq: &VecDeque<SequenceNode>, kmp_starting_position: usize) -> usize
    {
        let mut j = 0;
        let mut border = vec![0; seq.len() - kmp_starting_position as usize];

        for i in 1..seq.len() - kmp_starting_position as usize {
            while j > 0 && seq[i + kmp_starting_position as usize] != seq[j + kmp_starting_position as usize] {
                j = border[j - 1];
            }
            if seq[i + kmp_starting_position as usize] == seq[j + kmp_starting_position as usize] {
                j += 1;
            }
            border[i] = j;
        }

        j
    }

    
    /// render Nim values and store them in the sequence,
    /// ensuring that we stop the generation of Nim values at a clean point,
    /// in other words, at the border between two clumps of different Nim values;
    /// our checking (using the KMP algorithm) occurs elsewhere (not here)
    fn generate_new_values_for_sequence (mysequence: &mut VecDeque<SequenceNode>, x_cur: &mut SequenceCursor, y_cur: &mut SequenceCursor, z_cur: &mut SequenceCursor, new_length_generated: &mut u64, max_len: u64, current_len: &mut u64, kmp_starting_position: &mut usize) {
        loop {
            let mex = NimSequence::mexfunction(&[mysequence[x_cur.idx].value, mysequence[y_cur.idx].value, mysequence[z_cur.idx].value]);
            if *new_length_generated >= max_len && mysequence.back().unwrap().value != mex {
                // Only break when we have computed enough elements AND we are on
                // the border between two different values!
                break
            }

            NimSequence::add_new_element(mysequence, x_cur, y_cur, z_cur, mex, new_length_generated, current_len);
        
            // Trim sequence down to `max_len`.
            while *current_len - max_len >= mysequence[*kmp_starting_position].times_repeated {
                *current_len -= mysequence[*kmp_starting_position].times_repeated;
                *kmp_starting_position += 1;
            }
        }
    }

    /// print the full output of a Nim sequence, without any compression
    /// pub fn print_the_full_output_in_a_seq(&self, seq: &mut VecDeque<SeqNode>, myperiod: u64) {
    /// mychar is the character between each value and the times_repeated
    /// for instance, mychar could be * or ^ (as two natural examples)
    pub fn print_triplecheck_format( mypoint: Point, mychar: char ) {
        let mysequence = NimSequence::new( mypoint );
        let myseq = mysequence.seq;
        let myperiod = mysequence.periodlength;
        print!("x={} y={} z={} p={} ", mypoint.x, mypoint.y, mypoint.z, myperiod);
        for i in 0..myseq.len() {
            if myseq[i].value == u8::MAX {
                print!("( ");
            } else {
                print!("{}{}{} ", myseq[i].value, mychar, myseq[i].times_repeated);
            }
        }
        println!(")");
    }
    
    pub fn new( mypoint: Point ) -> Self {

        // need to actually build the nim sequence of values here
        let mut mysequence = VecDeque::new();
        let myperiod;

        let max_len = NimSequence::max_period_len( mypoint.x, mypoint.y, mypoint.z );
        let mut current_len = max_len;
        let mut kmp_starting_position = 0;

        // Prepopulate sequence with values far enough that any lookbehind will succeed.
        mysequence.push_back(SequenceNode::new(u8::MAX, max_len));

        let mut x_cur = SequenceCursor::new(max_len, mypoint.x);
        let mut y_cur = SequenceCursor::new(max_len, mypoint.y);
        let mut z_cur = SequenceCursor::new(max_len, mypoint.z);

        loop {
            // `new_length_generated` is the new length generated in this iteration.
            let mut new_length_generated = 0;

            NimSequence::generate_new_values_for_sequence(
                &mut mysequence,
                &mut x_cur,
                &mut y_cur,
                &mut z_cur,
                &mut new_length_generated,
                max_len,
                &mut current_len,
                &mut kmp_starting_position,
            );

            let len = NimSequence::kmp_preprocess_defined_start(&mysequence, kmp_starting_position);

            let mut actual_len = 0;
            for i in (mysequence.len() - len as usize)..mysequence.len() {
                actual_len += mysequence[i].times_repeated;
            }

            if actual_len >= mypoint.z {
                let runs_in_period = mysequence.len() - kmp_starting_position - len;
                myperiod = current_len - actual_len;

                while (mysequence.len() - 1 >= runs_in_period as usize)
                    && (mysequence[mysequence.len() - 1 - runs_in_period as usize] == mysequence[mysequence.len() - 1])
                {
                    mysequence.pop_back();
                }
                NimSequence::removing_excess_seq_length(&mut mysequence, myperiod);
                break;
            }
        }

        let mut mypreperiod = 0;
        let mut mycurrentposition = 0;
        while mysequence[mycurrentposition].value != u8::MAX {
            mypreperiod += mysequence[mycurrentposition].times_repeated;
            mycurrentposition += 1;
        }

        NimSequence { seq: mysequence, preperiodlength: mypreperiod, periodlength: myperiod }
    }

}


