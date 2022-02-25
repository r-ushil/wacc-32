.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(-4))), Sequence(Declaration(Int, "y", Expr(IntLiter(-2))), Println(BinaryApp(Ident("x"), Div, Ident("y"))))) }.generate(_, 4):
