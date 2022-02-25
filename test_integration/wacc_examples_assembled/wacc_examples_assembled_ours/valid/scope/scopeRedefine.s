.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(1))), Sequence(Scope(Sequence(Assignment(Ident("x"), Expr(IntLiter(2))), Sequence(Declaration(Bool, "x", Expr(BoolLiter(true))), Println(Ident("x"))))), Println(Ident("x")))) }.generate(_, 4):
