.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(15))), Sequence(Declaration(Int, "y", Expr(IntLiter(20))), Println(BinaryApp(Ident("y"), Sub, Ident("x"))))) }.generate(_, 4):