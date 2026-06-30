
// import modules
use subtractioncore::newutilities::*;
use subtractioncore::colortype::*;
use subtractioncore::point::*;
use subtractioncore::line::*;
use subtractioncore::matrix::*;
use subtractioncore::nimsequence::*;

//use egui::Color32;

fn main() {
    println!("Hello there");
    println!("The gcd of 24 and 100 is {}", gcd(24,100));
    let mycolor = Color::Orange;
    println!("The color is {:?}", to_color32(mycolor));
    println!("The point I want to study is {:?}", Point::new(1,5,19));
    println!("Divide the point by 7 to get {:?}", (Point::new(7,35,700) / 7).as_tuple());
    println!("Multiply the point by 3 to get {:?}", (Point::new(1,5,19) * 3).as_tuple());
    let mynewsequence = NimSequence::new( Point::new(2,4,7) );

    // let's say we want the fourth element nim value type:
    println!("The value type is {}", mynewsequence.seq[4].value);
    
    // and how many of them there are
    println!("The multiple is {}", mynewsequence.seq[4].times_repeated);
    
    
    println!("The preperiod length is {:?}", mynewsequence.preperiodlength);
    println!("The period length is {:?}", mynewsequence.periodlength);
    println!("The nim sequence is {:?}", mynewsequence.seq);
    
    println!("The line I want to study is {:?}", Line::new(-19,3,-110).as_string());
    println!("The line I want to study is {:?}", Line::new(i64::MAX,3,-110).as_string());
    println!("The point I want to study is {:?}", Point::new(u64::MAX,30,57).as_tuple());
    println!("The dot product is {}", Line::new(i64::MAX,i64::MAX,-110).dot(&Line::new(i64::MAX,-i64::MAX,57)));

    for x in 1..9 {
    for y in x+1..9 {
    for z in y+1..9 {
    if gcd(gcd(x,y),z) == 1 {        
//        println!("{} {} {}", x, y, z);
        let myvalues = NimSequence::new( Point::new(x,y,z) );
        let p = myvalues.periodlength;
        let pp = myvalues.preperiodlength;
        let mycolor = Point::new( x,y,z ).get_color();
        print!("x={} y={} z={} p={} pp={} color={:?} ", x, y, z, p, pp, mycolor);
        for i in 0..myvalues.seq.len() {
            if myvalues.seq[i].value == u8::MAX {   // this is the colon!
                print!("( ");
            }
            else {
                print!("{}*{} ", myvalues.seq[i].value, myvalues.seq[i].times_repeated);
            }
        }
        println!(")");
        
    }
    }
    }
    }

    println!("The matrix I want to study is {:?}", Matrix::new_from_point(Point::new(1,5,19), Point::new(3,17,100), Point::new(10,20,30)));

    println!("or it looks somewhat better like this {:?}", Matrix::new_from_point(Point::new(1,5,19), Point::new(3,17,100), Point::new(10,20,30)).as_string());

    println!("Now let's look at the point {:?}", Point::new(1,5,19).as_tuple());
    println!("Locally, it has orange line {:?}", Point::new(1,5,19).orange_line().as_string());
    println!("Locally, it has purple line {:?}", Point::new(1,5,19).purple_line().as_string());
    println!("Locally, it has green line {:?}", Point::new(1,5,19).green_line().as_string());

}

