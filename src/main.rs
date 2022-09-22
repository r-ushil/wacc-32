use wacc_32::compile_with_argv;

fn main() {
  let res = compile_with_argv();
  println!("{}\n{}", res.terminal_output(), res.asm_output());
}
