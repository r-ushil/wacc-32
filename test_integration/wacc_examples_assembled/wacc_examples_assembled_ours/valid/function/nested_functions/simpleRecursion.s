.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "rec", signature: FuncSig { params: [(Int, "x")], return_type: Int }, body: Sequence(If(BinaryApp(Ident("x"), Eq, IntLiter(0)), Skip, Declaration(Int, "y", Call("rec", [BinaryApp(Ident("x"), Sub, IntLiter(1))]))), Return(IntLiter(42))) }], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(0))), Assignment(Ident("x"), Call("rec", [IntLiter(8)]))) }.generate(_, 4):
