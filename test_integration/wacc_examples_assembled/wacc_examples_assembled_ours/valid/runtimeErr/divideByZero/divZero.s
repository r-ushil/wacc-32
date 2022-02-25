.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(BinaryApp(IntLiter(10), Div, IntLiter(0)))), Println(StrLiter("should not reach here"))) }.generate(_, 4):
