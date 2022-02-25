.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "f", signature: FuncSig { params: [(Int, "n")], return_type: Int }, body: Sequence(Declaration(Int, "i", Expr(IntLiter(0))), Sequence(While(BinaryApp(Ident("i"), Lt, Ident("n")), Assignment(Ident("i"), Expr(BinaryApp(Ident("i"), Add, IntLiter(1))))), Return(Ident("i")))) }], statement: Sequence(Declaration(Int, "x", Call("f", [IntLiter(10)])), Println(Ident("x"))) }.generate(_, 4):
