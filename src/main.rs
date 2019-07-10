extern crate rand;

use std::{
    env,
    fs,
    num::ParseIntError,
    iter::Iterator,
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
    let mut args = env::args().skip(1);
    let mut config = Config {
        dictionary: DICTIONARY.into(),
        separator: "-".into(),
        count: 3,
    };

    /* Example CLI args I'll accept.
     * -c4
     * -c 4
     * --count 4
     */
    while let Some(next_option) = args.next() {
        if next_option.starts_with("--") {
            do_long_option(&mut config, &next_option[2..], &mut args)?;
        } else if next_option.starts_with("-") {
            match &next_option[..2] {
                "-c" => { parse_count(&mut config, &next_option[2..], &mut args)?; },
                "-d" => { parse_dictionary(&mut config, &next_option[2..], &mut args)?; },
                "-s" => { parse_separator(&mut config, &next_option[2..], &mut args)?; },
                _ => return Err(format!("Invalid option passed: {}", &next_option[..1])),
            }
        } else {
            // Proof of below comment?
            continue; // Must not have been an option then.
        }
    }
    Ok(config)
}


fn do_long_option<I>(config: &mut Config, current_option: &str, options: &mut I) -> Result<(), String>
    where I: Iterator<Item=String>
{
    if current_option == "count" {
        match options.next() {
            Some(opt) => parse_count(config, &opt, options),
            None => Err(format!("Missing argument to count.")),
        }
    } else if current_option == "separator" {
        Ok(())
    } else if current_option == "dictionary" {
        Ok(())
    } else {
        Err(format!("Unknown option: {}", current_option))
    }
}

/* -c4
 * -c 4
 * --count 4
 */
fn parse_count<I>(config: &mut Config, current_option: &str, options: &mut I) -> Result<(), String>
    where I: Iterator<Item=String>
{
    if current_option == "" {
        match options.next() {
            Some(opt) => {
                let count: usize = opt.parse().map_err(|_| format!("Invalid number: {}", opt))?;
                config.update_word_count(count);
                Ok(())
            },
            None => Err(format!("Missing argument to count")),
        }
    } else {
        let count: usize = current_option.parse().map_err(|_| format!("Did you pass in a number? Got {}", current_option))?;
        config.update_word_count(count);
        Ok(())
    }
}

fn parse_dictionary<I>(config: &mut Config, current_option: &str, options: &mut I) -> Result<(), String>
    where I: Iterator<Item=String>
{
    Ok(())
}

fn parse_separator<I>(config: &mut Config, current_option: &str, options: &mut I) -> Result<(), String>
    where I: Iterator<Item=String>
{
    Ok(())
}

#[cfg(test)]
mod parse_count {

    use super::*;
    use std::iter;

    #[inline]
    fn default_config() -> Config {
        Config { count: 3, dictionary: String::new(), separator: String::new(), }
    }

    #[test]
    fn with_short_option_no_space() {
        let mut config = default_config();
        let mut args = iter::empty::<String>();
        // parse_count(..) expects "-c" to be removed before calling.
        // random_password -c4
        assert!(parse_count(&mut config, "4", &mut args).is_ok());
        assert_eq!(config.count, 4);
    }

    #[test]
    fn with_short_option_space() {
        let mut config = default_config();
        let mut args = vec![String::from("4")].into_iter();
        // parse_count(..) expects "-c" to be removed before calling.
        // random_password -c 4
        assert!(parse_count(&mut config, "", &mut args).is_ok());
        assert_eq!(config.count, 4);
    }

    #[test]
    fn with_long_option() {
        let mut config = default_config();
        let mut args = vec![String::from("4")].into_iter();
        // do_long_option(..) expects leading "--" to be removed before calling.
        assert!(do_long_option(&mut config, "count", &mut args).is_ok());
        assert_eq!(config.count, 4);
    }

    #[test]
    fn without_argument_to_short() {
        let mut config = default_config();
        let mut args = iter::empty::<String>();
        // parse_count(..) expects "-c" to be removed before calling.
        // random_password -c
        assert!(parse_count(&mut config, "", &mut args).is_err());
    }

    #[test]
    fn without_argument_to_long() {
        let mut config = default_config();
        let mut args = iter::empty::<String>();
        // do_long_option(..) expects leading "--" to be removed before calling.
        // random_password --count
        assert!(do_long_option(&mut config, "count", &mut args).is_err());
    }
}

#[cfg(test)]
mod parse_separator {

}

#[cfg(test)]
mod parse_dictionary {

}
