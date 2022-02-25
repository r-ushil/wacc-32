.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(5))), Sequence(Declaration(Int, "y", Expr(IntLiter(3))), Println(BinaryApp(Ident("x"), Mod, Ident("y"))))) }.generate(_, 4):
