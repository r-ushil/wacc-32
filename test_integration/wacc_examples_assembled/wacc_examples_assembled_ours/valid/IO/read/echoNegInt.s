.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(1))), Sequence(Println(StrLiter("enter an integer to echo")), Sequence(Read(Ident("x")), Println(Ident("x"))))) }.generate(_, 4):
