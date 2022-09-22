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

use wasm_bindgen::prelude::*;

/* WASM_bindgen stuff - need to return one object with getters for TypeScript */

#[wasm_bindgen]
pub struct CompileResult {
  terminal_output: String,
  asm_output: String,
}

#[wasm_bindgen]
impl CompileResult {
  #[wasm_bindgen(getter)]
  pub fn terminal_output(&self) -> String {
    self.terminal_output.clone()
  }

  #[wasm_bindgen(getter)]
  pub fn asm_output(&self) -> String {
    self.asm_output.clone()
  }
}

#[wasm_bindgen]
pub fn compile() -> CompileResult {
  // Get all arguments passed to the compiler
  let args: Vec<String> = env::args().collect();

  let mut terminal_output: Vec<String> = vec![];

  // Ensure that a single argument was given
  if args.len() < 3 {
    terminal_output.push(String::from("Error: note enough arguments."));
    print_usage(&mut terminal_output);
    exit(-1)
  }

  // Ensure that this argument is a path leading to file
  let source_path = &args[1];
  let destination_path = &args[2];
  if !Path::new(source_path).exists() {
    terminal_output.push(String::from("Error: input file does not exist."));
    print_usage(&mut terminal_output);
    exit(-1)
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

  let (ast, mut parse_output) = parse_with_terminal_output(program_str);

  terminal_output.append(&mut parse_output);

  match ast {
    None => CompileResult {
      terminal_output: terminal_output.join("\n"),
      asm_output: String::new(),
    },
    Some(mut ast) => {
      if !analyse(&mut ast, &mut terminal_output) {
        return CompileResult {
          terminal_output: terminal_output.join("\n"),
          asm_output: String::new(),
        };
      }

      if analysis_only {
        terminal_output.push(String::from("Halted after analysis stage."));
        return CompileResult {
          terminal_output: terminal_output.join("\n"),
          asm_output: String::new(),
        };
      }

      let code = generator::generate(&ast);
      let asm_output = write_asm(code, destination_path);
      terminal_output.push(String::from("Successful code generation."));

      CompileResult {
        terminal_output: terminal_output.join("\n"),
        asm_output,
      }
    }
  }
}

fn write_asm(code: GeneratedCode, destination_path: &str) -> String {
  let mut asm_text = String::new();
  write!(&mut asm_text, "{}", code).unwrap();
  fs::write(destination_path, asm_text.clone()).unwrap();
  asm_text
}

fn analyse(ast: &mut ast::Program, terminal_output: &mut Vec<String>) -> bool {
  match analyser::analyse(ast) {
    Ok(()) => {
      terminal_output.push(String::from("Successful semantic analysis."));
      true
    }
    Err(errors) => {
      terminal_output.push(errors.to_string());

      if contains_syntax_errors(&errors) {
        terminal_output.push(compile_error(100))
      } else {
        terminal_output.push(compile_error(200))
      }
      false
    }
  }
}

fn compile_error(code: u8) -> String {
  format!("Error: Code {}", code)
}

fn parse_with_terminal_output(
  program_str: &str,
) -> (Option<ast::Program>, Vec<String>) {
  let mut terminal_output = vec![];
  parse(program_str, &mut terminal_output)
}

fn parse(
  program_str: &str,
  terminal_output: &mut Vec<String>,
) -> (Option<ast::Program>, Vec<String>) {
  match parser::parse(program_str) {
    Ok(ast) => (Some(ast), terminal_output.to_vec()),
    Err(err_tree) => (
      None,
      pretty_print_err_tree(program_str, &err_tree, terminal_output),
    ),
  }
}

fn read_file(file: fs::File) -> String {
  let mut buf_reader = BufReader::new(file);
  let mut program_buf = String::new();
  buf_reader.read_to_string(&mut program_buf).unwrap();
  program_buf
}

fn contains_syntax_errors(errors: &SemanticError) -> bool {
  use SemanticError::*;
  match errors {
    Syntax(_) => true,
    Normal(_) => false,
    Join(e1, e2) => contains_syntax_errors(e1) || contains_syntax_errors(e2),
  }
}

const EXCERPT_SIZE: usize = 30;

fn pretty_print_err_tree(
  program: &str,
  err_tree: &ErrorTree<&str>,
  terminal_output: &mut Vec<String>,
) -> Vec<String> {
  match err_tree {
    ErrorTree::Base { location, kind } => {
      let context = Location::recreate_context(program, *location);

      let l = location.to_string();
      let l_len = l.len();

      let context_excerpt = &l[..EXCERPT_SIZE.min(l_len)];

      terminal_output.push(format!(
        "line {}, column {}: {} \nStart of error input: {}\n",
        context.line, context.column, kind, context_excerpt,
      ));
    }
    ErrorTree::Stack { base, contexts } => {
      for _ctx in contexts {
        pretty_print_err_tree(program, base, terminal_output);
      }
    }
    ErrorTree::Alt(errors) => {
      for error in errors {
        pretty_print_err_tree(program, error, terminal_output);
      }
    }
  }

  terminal_output.to_vec()
}

fn print_usage(terminal_output: &mut Vec<String>) {
  terminal_output.push(String::from(
    "Usage: ./wacc_32 <input_file_path> <output_file_path>",
  ))
}
