.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "f", signature: FuncSig { params: [], return_type: Int }, body: Sequence(Println(StrLiter("go")), Sequence(Return(IntLiter(1)), If(BoolLiter(true), Sequence(Println(StrLiter("a")), Return(IntLiter(2))), Sequence(Println(StrLiter("b")), Return(IntLiter(3)))))) }], statement: Sequence(Declaration(Int, "ret", Call("f", [])), Println(Ident("ret"))) }.generate(_, 4):
