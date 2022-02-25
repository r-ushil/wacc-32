.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Bool, "a", Expr(BoolLiter(true))), Sequence(Declaration(Bool, "b", Expr(BoolLiter(false))), Sequence(Println(UnaryApp(Bang, Ident("a"))), Println(UnaryApp(Bang, Ident("b")))))) }.generate(_, 4):
