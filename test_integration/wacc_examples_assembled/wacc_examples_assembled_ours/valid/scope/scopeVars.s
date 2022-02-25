.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(2))), Sequence(Println(Ident("x")), Sequence(Scope(Sequence(Declaration(Int, "x", Expr(IntLiter(4))), Println(Ident("x")))), Println(Ident("x"))))) }.generate(_, 4):
