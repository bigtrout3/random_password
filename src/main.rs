extern crate rand;
use rand::seq;

const DICTIONARY: &'static str = include_str!("../dictionary.txt");

fn main() {
    let words: Vec<_> = DICTIONARY.lines().collect();
    let mut rng = rand::thread_rng();
    let out: Vec<&str> = seq::sample_iter(
        &mut rng,
        words.into_iter(),
        3).unwrap();
    println!("{}", out.join("-"));
}
