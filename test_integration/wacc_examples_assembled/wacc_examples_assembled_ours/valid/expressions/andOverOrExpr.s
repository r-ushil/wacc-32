.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Bool, "a", Expr(BoolLiter(false))), Sequence(Declaration(Bool, "b", Expr(BoolLiter(false))), Sequence(Declaration(Bool, "c", Expr(BoolLiter(true))), Sequence(Println(BinaryApp(BinaryApp(Ident("a"), And, Ident("b")), Or, Ident("c"))), Println(BinaryApp(Ident("a"), And, BinaryApp(Ident("b"), Or, Ident("c")))))))) }.generate(_, 4):
