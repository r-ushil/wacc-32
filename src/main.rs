use wacc_32::compile;

fn main() {
  let (terminal_output, _asm) = compile();
  println!("{}", terminal_output);
}
