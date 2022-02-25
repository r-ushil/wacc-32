.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "f", signature: FuncSig { params: [(Int, "x")], return_type: Int }, body: Sequence(If(BinaryApp(Ident("x"), Eq, IntLiter(0)), Skip, Sequence(Declaration(Int, "i", Expr(Ident("x"))), Sequence(While(BinaryApp(Ident("i"), Gt, IntLiter(0)), Sequence(Print(StrLiter("-")), Assignment(Ident("i"), Expr(BinaryApp(Ident("i"), Sub, IntLiter(1)))))), Sequence(Println(StrLiter("")), Declaration(Int, "s", Call("f", [BinaryApp(Ident("x"), Sub, IntLiter(1))])))))), Return(IntLiter(0))) }], statement: Sequence(Println(StrLiter("Please enter the size of the triangle to print: ")), Sequence(Declaration(Int, "x", Expr(IntLiter(0))), Sequence(Read(Ident("x")), Declaration(Int, "s", Call("f", [Ident("x")]))))) }.generate(_, 4):
