mod ast;
mod parser;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::process::exit;

fn main() {
  // Get all arguments passed to the compiler
  let args: Vec<String> = env::args().collect();

  // Ensure that a single argument was given
  if args.len() != 2 {
    println!("Error: incorrect number of arguments. ");
    print_usage();
    exit(-1);
  }

  // Ensure that this argument is a path leading to file
  let source_path = &args[1];
  if !Path::new(source_path).exists() {
    println!("Error: file does not exist. ");
    print_usage();
    exit(-1);
  }

  // Read the contents of this file
  let file = File::open(source_path).unwrap();
  let mut buf_reader = BufReader::new(file);
  let mut program = String::new();
  buf_reader.read_to_string(&mut program).unwrap();

  // Attempt to parse the contents of this file, if an error encountered
  // then return it and exit with status 100
  let ast = match parser::parse(&program) {
    Ok(ast) => ast,
    Err(e) => {
      println!("{}", e);
      exit(100);
    }
  };

  // Print the generated abstract syntax tree
  println!("ast = {:?}", ast);
}

fn print_usage() {
  println!("Usage: ./wacc_32 <file_path>")
}
