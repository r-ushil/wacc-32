.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "f", signature: FuncSig { params: [(Int, "f")], return_type: Int }, body: Return(Ident("f")) }], statement: Sequence(Declaration(Int, "x", Call("f", [IntLiter(99)])), Println(Ident("x"))) }.generate(_, 4):
