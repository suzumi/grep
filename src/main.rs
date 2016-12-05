extern crate regex;
use regex::Regex;
use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn read_file(filename: String) -> Result<String, io::Error> {
  let mut file = try!(File::open(filename));
  let mut content = String::new();
  try!(file.read_to_string(&mut content));
  Ok(content)
}

fn extract_pattern(pattern: String, content: String) {
  let re = Regex::new(r"{}+", pattern).unwrap();
  let caps = re.captures(content).unwrap();
  println!("{}", caps.at(0).unwrap());
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() > 1 {

    println!("{}", match read_file(args[2].clone()) {
        Ok(content) => {
          extract_pattern(args[1], content)
        },
        Err(reason) => panic!("{}", reason)
    });
  }
}

