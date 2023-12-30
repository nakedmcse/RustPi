// Rust implementation of Leibniz
use std::time::Instant;

fn leibniz(iter:i64) -> f64 {
    let (mut n, mut topterm): (f64, f64) = (1.0, -1.0);
    let (mut bottomterm, mut term): (f64,f64);
    
    for i in 2..iter {
        bottomterm = (i as f64) * 2.0;
        term = topterm / (bottomterm - 1.0);
        n += term;
        topterm = -topterm;
    }

    return n * 4.0;
}

fn main() {
    println!("Starting Leibniz Approximation");
    let start_approx = Instant::now();
    let leibniz_pi = leibniz(10_000_000);
    let duration_approx = start_approx.elapsed();
    println!("Time: {0:?} Pi: {1}",duration_approx, leibniz_pi);
}