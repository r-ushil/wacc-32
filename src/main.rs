mod analyser;
mod ast;
mod parser;
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
  let mut program_buf = String::new();
  buf_reader.read_to_string(&mut program_buf).unwrap();

  let program = program_buf.as_str();

  // Attempt to parse the contents of this file, if an error encountered
  // then return it and exit with status 100
  let ast = match parser::parse(program) {
    Ok(ast) => ast,
    Err(err_tree) => {
      pretty_print_err_tree(program, &err_tree);
      exit(100);
    }
  };

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

const EXCERPT_SIZE: usize = 30;

fn pretty_print_err_tree(program: &str, err_tree: &ErrorTree<&str>) {
  match err_tree {
    ErrorTree::Base { location, kind } => {
      let context = Location::recreate_context(program, *location);

      let l = location.to_string();
      let l_len = l.len();

      let context_excerpt = &l[..EXCERPT_SIZE.min(l_len)];

      println!(
        "line {}, column {}: {} \nStart of error input: {}\n",
        context.line, context.column, kind, context_excerpt,
      );
    }
    ErrorTree::Stack { base, contexts } => {
      for _ctx in contexts {
        pretty_print_err_tree(program, base);
      }
    }
    ErrorTree::Alt(errors) => {
      for error in errors {
        pretty_print_err_tree(program, error);
      }
    }
  }
}

fn print_usage() {
  println!("Usage: ./wacc_32 <file_path>")
}
