.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "inc", signature: FuncSig { params: [(Int, "x")], return_type: Int }, body: Return(BinaryApp(Ident("x"), Add, IntLiter(1))) }], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(0))), Sequence(Assignment(Ident("x"), Call("inc", [Ident("x")])), Sequence(Println(Ident("x")), Sequence(Assignment(Ident("x"), Call("inc", [Ident("x")])), Sequence(Assignment(Ident("x"), Call("inc", [Ident("x")])), Sequence(Assignment(Ident("x"), Call("inc", [Ident("x")])), Println(Ident("x")))))))) }.generate(_, 4):
