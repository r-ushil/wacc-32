.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(42))), Sequence(Declaration(Int, "y", Expr(IntLiter(0))), Sequence(Println(Ident("x")), Println(Ident("y"))))) }.generate(_, 4):
