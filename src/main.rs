extern crate csv;
extern crate clap;
extern crate serde;
extern crate chrono;
#[macro_use]
extern crate serde_derive;

//use clap::{Arg,App};
use std::io::{stdout, BufWriter};
use ferris_says::say;
//use std::env;
use std::error::Error;
//use std::ffi::OsString;
//use std::fs::File;
use std::process;

mod record;

#[derive(Debug, Deserialize)]
struct CategoryMatcher {
    category: String,
    matcher: String
}

fn rust_say() {
    let stdout = stdout();
    let out = b"Hellow fellow Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}

fn get1() -> i32 {
    return 1;
}

fn get_records(file_path: &str) -> Result<Vec<record::Record>, Box<Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    Ok(rdr.deserialize().map(|x| x.unwrap()).collect())
}

fn run(file_path: &str) -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    for result in rdr.deserialize() {
        let record: record::Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    rust_say();
    if let Err(err) = run("_private/download.csv") {
        println!("{}", err);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn get1_works() {
        assert_eq!(get1(), 1);
    }

    #[test]
    fn run_works() {
        assert_eq!(run("_private/download.csv").unwrap(), ());
    }
}
