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
	SUB sp, sp, #3
	MOV r4, #'a'
	STRB r4, [sp, #2]
	MOV r4, #'e'
	STRB r4, [sp, #1]
	MOV r4, #'c'
	STRB r4, [sp]
	LDRSB r4, [sp, #2]
	LDRSB r5, [sp, #1]
	CMP r4, r5
	MOVLT r4, #1
	MOVGE r4, #0
	MOV r0, r4
	BL p_print_bool
	BL p_print_ln
	LDRSB r4, [sp, #1]
	LDRSB r5, [sp]
	CMP r4, r5
	MOVLT r4, #1
	MOVGE r4, #0
	MOV r0, r4
	BL p_print_bool
	BL p_print_ln
	ADD sp, sp, #3
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
