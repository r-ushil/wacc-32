.data:
.text:
.global main:
main:
Program { funcs: [Func { ident: "f", signature: FuncSig { params: [], return_type: Bool }, body: If(BoolLiter(true), Return(BoolLiter(true)), Return(BoolLiter(false))) }], statement: Sequence(Declaration(Bool, "x", Call("f", [])), Println(Ident("x"))) }.generate(_, 4):
