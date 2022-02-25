.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "fibonacci", signature: FuncSig { params: [(Int, "n")], return_type: Int }, body: Sequence(If(BinaryApp(Ident("n"), Lte, IntLiter(1)), Return(Ident("n")), Skip), Sequence(Declaration(Int, "f1", Call("fibonacci", [BinaryApp(Ident("n"), Sub, IntLiter(1))])), Sequence(Declaration(Int, "f2", Call("fibonacci", [BinaryApp(Ident("n"), Sub, IntLiter(2))])), Return(BinaryApp(Ident("f1"), Add, Ident("f2")))))) }], statement: Sequence(Println(StrLiter("This program calculates the nth fibonacci number recursively.")), Sequence(Print(StrLiter("Please enter n (should not be too large): ")), Sequence(Declaration(Int, "n", Expr(IntLiter(0))), Sequence(Read(Ident("n")), Sequence(Print(StrLiter("The input n is ")), Sequence(Println(Ident("n")), Sequence(Print(StrLiter("The nth fibonacci number is ")), Sequence(Declaration(Int, "result", Call("fibonacci", [Ident("n")])), Println(Ident("result")))))))))) }.generate(_, 4):
