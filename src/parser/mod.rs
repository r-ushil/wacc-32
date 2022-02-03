use std::error::Error;

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

pub fn parse<'a>(input: &'a str) -> Result<Program, Box<dyn Error + 'a>> {
  match program::program(input) {
    Ok((input, program)) if input == "" => Ok(program),
    Ok((input, _)) => Err(format!("Too much input, remaining input: {}", input).into()),
    Err(nom::Err::Failure(e)) | Err(nom::Err::Error(e)) => Err(Box::new(e)),
    _ => todo!(),
  }
}
