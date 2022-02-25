.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Bool, "b", Expr(BoolLiter(true))), Sequence(Declaration(Bool, "c", Expr(BoolLiter(false))), If(BinaryApp(Ident("b"), Or, Ident("c")), Println(StrLiter("correct")), Println(StrLiter("incorrect"))))) }.generate(_, 4):
