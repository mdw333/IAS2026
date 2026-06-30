//use eframe::egui;
//use egui::Color32;

/// the greatest common divisor of the two integers
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

