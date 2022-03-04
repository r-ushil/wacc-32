.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #1
	MOV r4, #1
	STRB r4, [sp]
	ADD sp, sp, #1
	LDR r0, =0
	POP {pc}
	.ltorg
