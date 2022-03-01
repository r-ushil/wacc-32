mod analyser;
mod ast;
mod generator;
mod parser;
use std::env;
use std::fmt::Write;
use std::fs;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::process::exit;

use analyser::SemanticError;
use generator::asm::GeneratedCode;
use nom_supreme::error::ErrorTree;
use nom_supreme::final_parser::Location;
use nom_supreme::final_parser::RecreateContext;

fn main() {
  // Get all arguments passed to the compiler
  let args: Vec<String> = env::args().collect();

  // Ensure that a single argument was given
  if args.len() < 3 {
    incorrect_usage("Error: not enough arguments. ")
  }

  // Ensure that this argument is a path leading to file
  let source_path = &args[1];
  let destination_path = &args[2];
  if !Path::new(source_path).exists() {
    incorrect_usage("Error: input file does not exist. ")
  }

  /* OPTIONS */
  // TODO: Accept these options before source and destination files
  // TODO: Multiple options
  // TODO: Options for long / short options
  // TODO: Update usage
  let mut analysis_only = false;
  if args.len() > 3 {
    // There are flags present

    // TODO: Shouldn't need to input output file if this
    // (or earlier termination) are desired
    if &args[3] == "--analysis" {
      analysis_only = true;
    }
  }

  // Read the contents of this file
  let program_string = read_file(fs::File::open(source_path).unwrap());
  let program_str = program_string.as_str();

  let mut ast = parse(program_str);
  analyse(&mut ast);
  let code = generator::generate(&ast);

  if analysis_only {
    println!("Halted after analysis stage. ");
    exit(0);
  }

  let code = generator::generate(&ast);
  write_asm(code, destination_path);
}

fn write_asm(code: GeneratedCode, destination_path: &str) {
  let mut asm_text = String::new();
  write!(&mut asm_text, "{}", code).unwrap();
  fs::write(destination_path, asm_text).unwrap();
}

fn analyse(ast: &mut ast::Program) {
  match analyser::analyse(ast) {
    Ok(()) => {
      println!("Successful semantic analysis.");
    }
    Err(errors) => {
      print_semantic_errors(&errors);
      if contains_syntax_errors(&errors) {
        exit(100);
      } else {
        exit(200);
      }
    }
  }
}

fn parse(program_str: &str) -> ast::Program {
  match parser::parse(program_str) {
    Ok(ast) => ast,
    Err(err_tree) => {
      pretty_print_err_tree(program_str, &err_tree);
      exit(100);
    }
  }
}

fn read_file(file: fs::File) -> String {
  let mut buf_reader = BufReader::new(file);
  let mut program_buf = String::new();
  buf_reader.read_to_string(&mut program_buf).unwrap();
  program_buf
}

fn incorrect_usage(reason: &str) {
  println!("{}", reason);
  print_usage();
  exit(-1);
}

fn print_semantic_errors(errors: &Vec<SemanticError>) {
  for error in errors {
    println!("ERROR: {}", error);
  }
}

fn contains_syntax_errors(errors: &Vec<SemanticError>) -> bool {
  for error in errors {
    if is_syntax(&error) {
      return true;
    }
  }
  false
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

fn is_syntax(error: &SemanticError) -> bool {
  match error {
    SemanticError::Syntax(_) => true,
    SemanticError::Normal(_) => false,
    SemanticError::Nested(_, b) => is_syntax(&*b),
  }
}

fn print_usage() {
  println!("Usage: ./wacc_32 <input_file_path> <output_file_path>")
}
