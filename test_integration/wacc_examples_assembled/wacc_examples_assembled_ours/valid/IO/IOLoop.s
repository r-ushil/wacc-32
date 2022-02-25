.data:
.text:
.global main:
main:
Program { funcs: [], statement: Sequence(Declaration(Char, "continue", Expr(CharLiter('Y'))), Sequence(Declaration(Int, "buff", Expr(IntLiter(0))), While(BinaryApp(Ident("continue"), Neq, CharLiter('N')), Sequence(Print(StrLiter("Please input an integer: ")), Sequence(Read(Ident("buff")), Sequence(Print(StrLiter("echo input: ")), Sequence(Println(Ident("buff")), Sequence(Println(StrLiter("Do you want to continue entering input?")), Sequence(Println(StrLiter("(enter Y for 'yes' and N for 'no')")), Read(Ident("continue"))))))))))) }.generate(_, 4):
