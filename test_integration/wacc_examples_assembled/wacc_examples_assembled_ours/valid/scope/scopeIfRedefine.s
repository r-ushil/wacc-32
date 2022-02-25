.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(12))), Sequence(If(BinaryApp(Ident("x"), Eq, IntLiter(12)), Sequence(Declaration(Bool, "x", Expr(BoolLiter(true))), Println(Ident("x"))), Sequence(Declaration(Char, "x", Expr(CharLiter('a'))), Println(Ident("x")))), Println(Ident("x")))) }.generate(_, 4):
