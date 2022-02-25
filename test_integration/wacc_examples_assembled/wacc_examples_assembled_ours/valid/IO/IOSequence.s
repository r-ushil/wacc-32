.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(0))), Sequence(Print(StrLiter("Please input an integer: ")), Sequence(Read(Ident("x")), Sequence(Print(StrLiter("You input: ")), Println(Ident("x")))))) }.generate(_, 4):
