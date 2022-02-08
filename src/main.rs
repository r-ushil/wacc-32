mod ast;
mod parser;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::process::exit;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    println!("Error: incorrect number of arguments. ");
    // TODO: Print usage
    exit(-1);
  }

  let source_path = &args[1];

  if !Path::new(source_path).exists() {
    println!("Error: file does not exist. ");
    // TODO: Print usage
    exit(-1);
  }

  let file = File::open(source_path).unwrap();

  let mut buf_reader = BufReader::new(file);
  let mut program = String::new();
  buf_reader.read_to_string(&mut program).unwrap();

  // Parse the program
  // let ast = parser::parse(program);

  //TODO Handle errors from parsing

  // println!("ast = {:?}", ast);
}
