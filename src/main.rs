mod ast;
mod parser;
use std::env;
use std::process::exit;

fn main() {
  let args: Vec<String> = env::args().collect();
  let argc = args.len();
  // Check the correct number of arguments
  if argc != 2 {
    // TODO: Print correct usage of the command
    println!("Usage here. ");
    exit(-1);
  }

  let source_path = &args[1];
  println!("Source path: {}", source_path);

  // TODO: Check the given path is a file that exists

  // TODO: Load the file contents into the program string

  // Parse the program
  // let ast = parser::parse(program);

  //TODO Handle errors from parsing

  // println!("ast = {:?}", ast);
}
