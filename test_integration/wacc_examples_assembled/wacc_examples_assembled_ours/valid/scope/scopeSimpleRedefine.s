.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(12))), Sequence(Scope(Sequence(Declaration(Bool, "x", Expr(BoolLiter(true))), Println(Ident("x")))), Println(Ident("x")))) }.generate(_, 4):
