.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "printLine", signature: FuncSig { params: [(Int, "n")], return_type: Bool }, body: Sequence(Declaration(Int, "i", Expr(IntLiter(0))), Sequence(While(BinaryApp(Ident("i"), Lt, Ident("n")), Sequence(Print(StrLiter("-")), Assignment(Ident("i"), Expr(BinaryApp(Ident("i"), Add, IntLiter(1)))))), Sequence(Println(StrLiter("")), Return(BoolLiter(true))))) }, Func { ident: "printMap", signature: FuncSig { params: [(Int, "n")], return_type: Bool }, body: Sequence(Print(StrLiter("|  ")), Sequence(If(BinaryApp(Ident("n"), Lt, IntLiter(100)), Print(StrLiter(" ")), Skip), Sequence(Print(Ident("n")), Sequence(Print(StrLiter(" = ")), Sequence(Print(UnaryApp(Chr, Ident("n"))), Sequence(Println(StrLiter("  |")), Return(BoolLiter(true)))))))) }], statement: Sequence(Println(StrLiter("Ascii character lookup table:")), Sequence(Declaration(Bool, "r", Call("printLine", [IntLiter(13)])), Sequence(Declaration(Int, "num", Expr(UnaryApp(Ord, CharLiter(' ')))), Sequence(While(BinaryApp(Ident("num"), Lt, IntLiter(127)), Sequence(Assignment(Ident("r"), Call("printMap", [Ident("num")])), Assignment(Ident("num"), Expr(BinaryApp(Ident("num"), Add, IntLiter(1)))))), Assignment(Ident("r"), Call("printLine", [IntLiter(13)])))))) }.generate(_, 4):