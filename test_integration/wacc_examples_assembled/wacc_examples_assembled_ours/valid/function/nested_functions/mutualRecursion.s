.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "r1", signature: FuncSig { params: [(Int, "x")], return_type: Int }, body: Sequence(If(BinaryApp(Ident("x"), Eq, IntLiter(0)), Skip, Sequence(Print(StrLiter("r1: sending ")), Sequence(Println(Ident("x")), Declaration(Int, "y", Call("r2", [Ident("x")]))))), Return(IntLiter(42))) }, Func { ident: "r2", signature: FuncSig { params: [(Int, "y")], return_type: Int }, body: Sequence(Print(StrLiter("r2: received ")), Sequence(Println(Ident("y")), Sequence(Declaration(Int, "z", Call("r1", [BinaryApp(Ident("y"), Sub, IntLiter(1))])), Return(IntLiter(44))))) }], statement: Sequence(Declaration(Int, "x", Expr(IntLiter(0))), Assignment(Ident("x"), Call("r1", [IntLiter(8)]))) }.generate(_, 4):
