.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "f", signature: FuncSig { params: [(Int, "x")], return_type: Int }, body: Sequence(If(BinaryApp(Ident("x"), Eq, IntLiter(0)), Skip, Sequence(Declaration(Int, "i", Expr(Ident("x"))), Sequence(While(BinaryApp(Ident("i"), Gt, IntLiter(0)), Sequence(Print(StrLiter("-")), Assignment(Ident("i"), Expr(BinaryApp(Ident("i"), Sub, IntLiter(1)))))), Sequence(Println(StrLiter("")), Declaration(Int, "s", Call("f", [BinaryApp(Ident("x"), Sub, IntLiter(1))])))))), Return(IntLiter(0))) }], statement: Declaration(Int, "s", Call("f", [IntLiter(8)])) }.generate(_, 4):
