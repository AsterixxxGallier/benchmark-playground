use crate::benchmark::benchmark;
use rand::random;

mod benchmark;

fn main() {
    benchmark(|| random(), |secret: u64| secret * secret).report();
}
