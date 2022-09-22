use wacc_32::compile;

fn main() {
  let (terminal_output, asm) = compile();
  println!("{}\n{}", terminal_output, asm);
}
