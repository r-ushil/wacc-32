.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "f", signature: FuncSig { params: [(Int, "x")], return_type: Int }, body: Sequence(Print(StrLiter("x is ")), Sequence(Println(Ident("x")), Sequence(Assignment(Ident("x"), Expr(IntLiter(5))), Sequence(Print(StrLiter("x is now ")), Sequence(Println(Ident("x")), Return(Ident("x"))))))) }], statement: Sequence(Declaration(Int, "y", Expr(IntLiter(1))), Sequence(Print(StrLiter("y is ")), Sequence(Println(Ident("y")), Sequence(Declaration(Int, "x", Call("f", [Ident("y")])), Sequence(Print(StrLiter("y is still ")), Println(Ident("y"))))))) }.generate(_, 4):
