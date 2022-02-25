.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(String, "s1", Expr(StrLiter("Hi"))), Sequence(Declaration(String, "s2", Expr(StrLiter("Hello"))), Sequence(Print(StrLiter("s1 is ")), Sequence(Println(Ident("s1")), Sequence(Print(StrLiter("s2 is ")), Sequence(Println(Ident("s2")), Sequence(If(BinaryApp(Ident("s1"), Eq, Ident("s2")), Println(StrLiter("They are the same string.")), Println(StrLiter("They are not the same string."))), Sequence(Println(StrLiter("Now make s1 = s2")), Sequence(Assignment(Ident("s1"), Expr(Ident("s2"))), Sequence(Print(StrLiter("s1 is ")), Sequence(Println(Ident("s1")), Sequence(Print(StrLiter("s2 is ")), Sequence(Println(Ident("s2")), If(BinaryApp(Ident("s1"), Eq, Ident("s2")), Println(StrLiter("They are the same string.")), Println(StrLiter("They are not the same string.")))))))))))))))) }.generate(_, 4):
