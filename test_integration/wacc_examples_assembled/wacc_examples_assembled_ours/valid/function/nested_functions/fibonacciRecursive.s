.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "fibonacci", signature: FuncSig { params: [(Int, "n"), (Bool, "toPrint")], return_type: Int }, body: Sequence(If(BinaryApp(Ident("n"), Lte, IntLiter(1)), Return(Ident("n")), Skip), Sequence(Declaration(Int, "f1", Call("fibonacci", [BinaryApp(Ident("n"), Sub, IntLiter(1)), Ident("toPrint")])), Sequence(If(Ident("toPrint"), Sequence(Print(Ident("f1")), Print(StrLiter(", "))), Skip), Sequence(Declaration(Int, "f2", Call("fibonacci", [BinaryApp(Ident("n"), Sub, IntLiter(2)), BoolLiter(false)])), Return(BinaryApp(Ident("f1"), Add, Ident("f2"))))))) }], statement: Sequence(Println(StrLiter("The first 20 fibonacci numbers are:")), Sequence(Print(StrLiter("0, ")), Sequence(Declaration(Int, "result", Call("fibonacci", [IntLiter(19), BoolLiter(true)])), Sequence(Print(Ident("result")), Println(StrLiter("...")))))) }.generate(_, 4):
