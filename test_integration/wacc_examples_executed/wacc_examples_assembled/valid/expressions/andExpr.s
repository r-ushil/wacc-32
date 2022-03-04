.data

msg_0:
	.word 5
	.ascii	"true\0"
msg_1:
	.word 6
	.ascii	"false\0"
msg_2:
	.word 1
	.ascii	"\0"

.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #2
	MOV r4, #1
	STRB r4, [sp, #1]
	MOV r4, #0
	STRB r4, [sp]
	LDRSB r4, [sp, #1]
	LDRSB r5, [sp]
	AND r4, r4, r5
	MOV r0, r4
	BL p_print_bool
	BL p_print_ln
	LDRSB r4, [sp, #1]
	MOV r5, #1
	AND r4, r4, r5
	MOV r0, r4
	BL p_print_bool
	BL p_print_ln
	LDRSB r4, [sp]
	MOV r5, #0
	AND r4, r4, r5
	MOV r0, r4
	BL p_print_bool
	BL p_print_ln
	ADD sp, sp, #2
	LDR r0, =0
	POP {pc}
	.ltorg
p_print_bool:
	PUSH {lr}
	CMP r0, #0
	LDRNE r0, =msg_0
	LDREQ r0, =msg_1
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_2
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
