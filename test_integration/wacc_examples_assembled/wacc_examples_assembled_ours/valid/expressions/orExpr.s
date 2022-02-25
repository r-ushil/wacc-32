.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Bool, "a", Expr(BoolLiter(true))), Sequence(Declaration(Bool, "b", Expr(BoolLiter(false))), Sequence(Println(BinaryApp(Ident("a"), Or, Ident("b"))), Sequence(Println(BinaryApp(Ident("a"), Or, BoolLiter(true))), Println(BinaryApp(Ident("b"), Or, BoolLiter(false))))))) }.generate(_, 4):
