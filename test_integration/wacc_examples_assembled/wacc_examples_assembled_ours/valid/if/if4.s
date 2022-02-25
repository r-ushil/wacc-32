.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Bool, "b", Expr(BoolLiter(true))), Sequence(Declaration(Bool, "c", Expr(BoolLiter(false))), If(BinaryApp(Ident("b"), And, Ident("c")), Println(StrLiter("incorrect")), Println(StrLiter("correct"))))) }.generate(_, 4):
