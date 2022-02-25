.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Char, "c1", Expr(CharLiter('a'))), Sequence(Declaration(Char, "c2", Expr(CharLiter('z'))), Sequence(Println(BinaryApp(Ident("c1"), Eq, Ident("c2"))), Sequence(Println(BinaryApp(Ident("c1"), Neq, Ident("c2"))), Sequence(Println(BinaryApp(Ident("c1"), Lt, Ident("c2"))), Sequence(Println(BinaryApp(Ident("c1"), Lte, Ident("c2"))), Sequence(Println(BinaryApp(Ident("c1"), Gt, Ident("c2"))), Println(BinaryApp(Ident("c1"), Gte, Ident("c2")))))))))) }.generate(_, 4):
