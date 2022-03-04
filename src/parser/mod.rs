use nom_supreme::error::ErrorTree;

use crate::ast::Program;

mod expr;
mod program;
mod shared;
mod stat;
mod type_;

/* ======= PARSERS ======= */
/* The parser which parses a string into a AST node of type
ExampleNode, will have the name example_node. */
/* If names conflict with Rust keywords, an underscore is appended. */
/* All parsers will consume all leading whitespace before and after parsing. */

pub fn parse(input: &str) -> Result<Program, ErrorTree<&'_ str>> {
  match program::final_program_parser(input) {
    Ok(program) => Ok(program),
    Err(e) => Err(e),
  }
}
