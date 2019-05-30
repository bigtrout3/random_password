use std::env;
use std::fs::File;
use std::io::Read;

extern crate rand;
use rand::seq;

const DICTIONARY: &'static str = include_str!("../dictionary.txt");

struct Config {
    dictionary: String,
    separator: String,
    nwords: usize,
}

fn main() {
    let conf = cli();
    make_password(conf);
}

fn make_password(conf: Config) {
    let words: Vec<_> = conf.dictionary.lines().collect();
    let mut rng = rand::thread_rng();
    let out: Vec<&str> = seq::sample_iter(
        &mut rng,
        words.into_iter(),
        conf.nwords).unwrap();
    println!("{}", out.join(&conf.separator));
}

fn cli() -> Config {
    let mut args = env::args().skip(1);
    let mut c = Config {
        dictionary: DICTIONARY.into(),
        separator: "-".into(),
        nwords: 3,
    };

    loop {
        match args.next() {
            Some(s) => {
                match s.as_str() {
                    "-n" => {
                        c.nwords = args.next()
                        .expect("number of words to make")
                        .parse().expect("valid number");
                    },
                    "-s" => {
                        c.separator = args.next()
                            .expect("string separator");
                    },
                    "-d" => {
                        let f = args.next().expect("a filename on command line");
                        let mut s = String::new();
                        File::open(f)
                            .expect("specified file to exist")
                            .read_to_string(&mut s).unwrap();
                        c.dictionary = s;
                    }
                    _ => println!("what"),
                }
            },
            None => break,
        }
    }
    c
}
