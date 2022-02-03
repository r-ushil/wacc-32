mod ast;
mod parser;

fn main() {
  let program = "begin\n
    int foo(int x) is
      return x
    end
    
    int y = call foo(5 + 1)
  end";

  let ast = parser::parse(program);

  assert!(ast.is_ok());

  println!("ast = {:?}", ast);
}
