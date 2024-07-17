use std::io::prelude::BufRead;
use std::collections::HashMap;
use snafu::{prelude::*, ErrorCompat};
mod errors;

// #[derive(Debug)]
// struct WordCounter(HashMap<String, u64>);
// impl WordCounter {
//     fn new() -> WordCounter {
//         WordCounter(HashMap::new())
//     }
//     fn increment(&mut self, word: &str) {
//         let key = word.to_string();
//         let count = self.0.entry(key).or_insert(0);
//         *count += 1;
//     }
//     fn display(&self) {
//         for (key, value) in self.0.iter() {
//             println!("{}: {}", key, value);
//         }
//     }
// }
#[derive(Debug, Snafu)]
#[snafu(display("ID may not be less than 10, but it was {id}"))]
struct InvalidIdError {
    id: u16,
}
#[derive(Debug, Snafu)]
#[snafu(display("Invalid string: {s}"))]
#[snafu(context(suffix(Con)))]
struct InvalidString{
    s : String,
}

// fn is_valid_id(id: u16) -> Result<(), InvalidIdError> {
//     ensure!(id >= 10, InvalidIdSnafu { id });
//     Ok(())
// }
fn is_valid_string(s: String) -> Result<(), InvalidString> {
    ensure!(s.len() >= 10, InvalidStringCon { s });
    Ok(())
}
#[derive(Debug, Snafu)]
enum Error {
    UsesTheDefaultSuffixError,

    #[snafu(context(suffix(Con)))]
    HasAnotherSuffix,

    #[snafu(context(suffix(false)))]
    DoesNotHaveASuffix,
}

fn my_code() -> Result<(), Error> {
    UsesTheDefaultSuffixSnafu.fail()?;

    HasAnotherSuffixCon.fail()?;

    DoesNotHaveASuffix.fail()?;

    Ok(())
}
fn main() {
    if let Err(e) = my_code() {
        eprintln!("An error occurred: {e}");
        if let Err(err) = errors::is_valid_id(5) {
            eprintln!("Error: {err}");
        }
    }
}
