.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(42))), Println(UnaryApp(Neg, Ident("x")))) }.generate(_, 4):
