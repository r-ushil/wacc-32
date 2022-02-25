.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(String, "s1", Expr(StrLiter("Hello"))), Sequence(Declaration(String, "s2", Expr(StrLiter("foo"))), Sequence(Declaration(String, "s3", Expr(StrLiter("bar"))), Sequence(Declaration(Bool, "b", Expr(BinaryApp(Ident("s1"), Eq, Ident("s1")))), Sequence(Println(Ident("b")), Sequence(Println(BinaryApp(Ident("s1"), Eq, Ident("s2"))), Println(BinaryApp(Ident("s2"), Eq, Ident("s3"))))))))) }.generate(_, 4):
