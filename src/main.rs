extern crate rand;

use std::{
    env,
    fs,
    num::ParseIntError,
};
use rand::seq;

const DICTIONARY: &'static str = include_str!("../dictionary.txt");

#[derive(Debug)]
struct Config {
    dictionary: String,
    separator: String,
    count: usize,
}

impl Config {
    #[inline]
    fn update_word_count(&mut self, c: usize) {
        self.count = c;
    }

    #[inline]
    fn update_dictionary(&mut self, d: String) {
        self.dictionary = d;
    }

    #[inline]
    fn update_separator(&mut self, s: String) {
        self.separator = s;
    }
}

fn main() -> Result<(), String>{
    let conf = cli()?;
    make_password(conf);
    Ok(())
}

fn make_password(conf: Config) {
    let words: Vec<_> = conf.dictionary.lines().collect();
    let mut rng = rand::thread_rng();
    let out: Vec<&str> = seq::sample_iter(
        &mut rng,
        words.into_iter(),
        conf.count).unwrap();
    println!("{}", out.join(&conf.separator));
}

fn cli() -> Result<Config, String> {
    // let mut args = env::args().skip(1);
    let args: String = env::args().skip(1).fold(
        String::new(),
        |mut acc, arg| {
            acc.push(' ');
            acc.push_str(&arg);
            acc
        });
    let mut config = Config {
        dictionary: DICTIONARY.into(),
        separator: "-".into(),
        count: 3,
    };

    /* Example CLI args I'll accept.
     * -c4
     * -c 4
     * --count 4
     * --count=4
     */
    let mut current_char = 0;
    while let Some(next_option) = args[current_char..].chars().position(|c| c == '-') {
        let position = current_char + next_option;
        match args[position+1..].chars().next() {
            Some('-') => { do_long_option(&mut config, &args[position+2..])?; },
            Some('c') => { parse_count(&mut config, &args[position+2..])?; },
            Some('s') => { parse_separator(&mut config, &args[position+2..])?; },
            Some('d') => { parse_dictionary(&mut config, &args[position+2..])?; },
            _ => return Err(String::from("Invalid option passed")),
        }
        current_char = position+2;
    }
    Ok(config)
}


fn do_long_option(config: &mut Config, option: &str) -> Result<(), String> {
    if option.starts_with("count") {
        parse_count(config, &option["count".len()..])
    } else if option.starts_with("separator") {
        parse_separator(config, &option["separator".len()..])
    } else if option.starts_with("dictionary") {
        parse_dictionary(config, &option["dictionary".len()..])
    } else {
        Err(String::from("Invalid long option passed"))
    }
}

fn parse_count(config: &mut Config, option: &str) -> Result<(), String> {
    let count = option.chars()
        .skip_while(|&c| c.is_whitespace() || c == '=')
        .take_while(|c| c.is_digit(10) )
        .collect::<String>();
    let c: usize = count.parse().map_err(|e: ParseIntError| e.to_string())?;
    config.update_word_count(c);
    Ok(())
}

fn parse_dictionary(config: &mut Config, option: &str) -> Result<(), String> {
    let file_path = option.chars()
        .skip_while(|&c| c.is_whitespace() || c == '=')
        .take_while(|c| c.is_alphanumeric() )
        .collect::<String>();
    let dictionary = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    config.update_dictionary(dictionary);
    Ok(())
}

fn parse_separator(config: &mut Config, option: &str) -> Result<(), String> {
    let separator = option.chars()
        .skip_while(|&c| c.is_whitespace() || c == '=')
        .take_while(|c| !c.is_whitespace() )
        .collect::<String>();
    config.update_separator(separator);
    Ok(())
}
