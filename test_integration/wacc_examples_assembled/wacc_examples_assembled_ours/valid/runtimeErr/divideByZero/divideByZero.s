.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(10))), Sequence(Declaration(Int, "y", Expr(IntLiter(0))), Print(BinaryApp(Ident("x"), Div, Ident("y"))))) }.generate(_, 4):
