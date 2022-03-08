.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, =5
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	BL exit
	ADD sp, sp, #4
	LDR r0, =0
	POP {pc}
	.ltorg
