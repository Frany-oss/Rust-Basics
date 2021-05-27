
fn fifth_rooot(x: f64) -> f64 {
    x.powf(1f64 / 5f64)
}
fn main() {
    println!("Hello, world!");

    let mut sum: i64 = 0;

    for n in 0..100 {
        if n % 2 == 0 {
            sum += n* n;
        }
    }

    println!("%5th root of {}: {}", sum, fifth_rooot(sum as f64));
}
