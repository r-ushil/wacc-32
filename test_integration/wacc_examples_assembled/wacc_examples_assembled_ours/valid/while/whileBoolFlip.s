.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Bool, "b", Expr(BoolLiter(true))), Sequence(While(Ident("b"), Sequence(Println(StrLiter("flip b!")), Assignment(Ident("b"), Expr(UnaryApp(Bang, Ident("b")))))), Println(StrLiter("end of loop")))) }.generate(_, 4):
