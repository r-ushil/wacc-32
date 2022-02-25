.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "a", Expr(IntLiter(13))), Sequence(Declaration(Int, "b", Expr(IntLiter(37))), If(BinaryApp(Ident("a"), Lt, Ident("b")), Println(StrLiter("correct")), Println(StrLiter("incorrect"))))) }.generate(_, 4):
