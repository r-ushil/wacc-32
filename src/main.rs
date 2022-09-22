use wacc_32::compile;

fn main() {
  let res = compile();
  println!("{}\n{}", res.terminal_output(), res.asm_output());
}
