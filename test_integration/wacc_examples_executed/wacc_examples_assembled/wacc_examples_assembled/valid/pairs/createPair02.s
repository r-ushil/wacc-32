.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r0, =8
	BL malloc
	MOV r4, r0
	MOV r5, #'a'
	LDR r0, =1
	BL malloc
	STRB r5, [r0]
	STR r0, [r4]
	MOV r5, #'b'
	LDR r0, =1
	BL malloc
	STRB r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp]
	ADD sp, sp, #4
	LDR r0, =0
	POP {pc}
	.ltorg
