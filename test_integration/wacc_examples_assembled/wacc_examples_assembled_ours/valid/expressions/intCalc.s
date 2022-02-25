.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(42))), Sequence(Declaration(Int, "y", Expr(IntLiter(30))), Sequence(Declaration(Int, "z", Expr(BinaryApp(Ident("x"), Add, Ident("y")))), Println(Ident("z"))))) }.generate(_, 4):
