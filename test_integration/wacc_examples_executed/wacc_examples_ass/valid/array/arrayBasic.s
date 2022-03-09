.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, =0
	STR r5, [r4, #4]
	LDR r5, =1
	STR r5, [r4]
	STR r4, [sp]
	ADD sp, sp, #4
	LDR r0, =0
	POP {pc}
	.ltorg
