mod analyser;
mod ast;
mod parser;
use std::cmp::min;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::process::exit;

use analyser::SemanticError;
use nom_supreme::error::ErrorTree;
use nom_supreme::final_parser::Location;
use nom_supreme::final_parser::RecreateContext;

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

  let p_str = program.as_str();

  // Attempt to parse the contents of this file, if an error encountered
  // then return it and exit with status 100
  let ast = match parser::parse(p_str) {
    Ok(ast) => ast,
    Err(e) => {
      pretty_print(p_str, &e);
      exit(100);
    }
  };

  fn pretty_print(program: &str, err_tree: &ErrorTree<&str>) {
    match err_tree {
      ErrorTree::Base { location, kind } => {
        let context = Location::recreate_context(program, *location);
        let location_str = location.to_string();
        let length = location_str.len();
        let bound = 30.min(length);
        println!(
          "line {}, column {}: {} \nStart of error input: {}\n",
          context.line,
          context.column,
          kind,
          &location_str[..bound]
        );
      }
      ErrorTree::Stack { base, contexts } => {
        for _ctx in contexts {
          pretty_print(program, base);
        }
      }
      ErrorTree::Alt(errors) => {
        for error in errors {
          pretty_print(program, error);
        }
      }
    }
  }

  // Print the generated abstract syntax tree
  // println!("ast = {:?}", ast);

  match analyser::analyse(&ast) {
    Ok(()) => {
      println!("Successful semantic analysis.");
      exit(0);
    }
    Err(SemanticError::Syntax(e)) => {
      println!("SYNTAX ERROR: at line: and column: {}", e);
      exit(100);
    }
    Err(SemanticError::Normal(e)) => {
      println!("SEMANTIC ERROR: {}", e);
      exit(200);
    }
  }
}

fn print_usage() {
  println!("Usage: ./wacc_32 <file_path>")
}
