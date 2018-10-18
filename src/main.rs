extern crate rand;
use rand::seq;

const DICTIONARY: &'static str = include_str!("../dictionary.txt");

struct Config<'a> {
    dictionary: &'a str,
    separator: &'a str,
    nwords: usize,
}

fn main() {
    let conf = Config {
        dictionary: DICTIONARY,
        separator: "-",
        nwords: 3,
    };
    make_password(conf);
}

fn make_password(conf: Config) -> () {
    let words: Vec<_> = conf.dictionary.lines().collect();
    let mut rng = rand::thread_rng();
    let out: Vec<&str> = seq::sample_iter(
        &mut rng,
        words.into_iter(),
        conf.nwords).unwrap();
    println!("{}", out.join(conf.separator));
}
