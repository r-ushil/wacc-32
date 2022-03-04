.text

.global main
f_f:
	PUSH {lr}
	LDR r4, =0
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
main:
	PUSH {lr}
	LDR r0, =0
	POP {pc}
	.ltorg
