.data

msg_0:
	.word 2
	.ascii	"Hi"
msg_1:
	.word 5
	.ascii	"Hello"
msg_2:
	.word 6
	.ascii	"s1 is "
msg_3:
	.word 6
	.ascii	"s2 is "
msg_4:
	.word 25
	.ascii	"They are the same string."
msg_5:
	.word 29
	.ascii	"They are not the same string."
msg_6:
	.word 16
	.ascii	"Now make s1 = s2"
msg_7:
	.word 6
	.ascii	"s1 is "
msg_8:
	.word 6
	.ascii	"s2 is "
msg_9:
	.word 25
	.ascii	"They are the same string."
msg_10:
	.word 29
	.ascii	"They are not the same string."
msg_11:
	.word 5
	.ascii	"%.*s\0"
msg_12:
	.word 1
	.ascii	"\0"

.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r4, =msg_0
	STR r4, [sp, #4]
	LDR r4, =msg_1
	STR r4, [sp]
	LDR r4, =msg_2
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_3
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp]
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, [sp, #4]
	LDR r5, [sp]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L0
	LDR r4, =msg_4
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	B L1
L0:
	LDR r4, =msg_5
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
L1:
	LDR r4, =msg_6
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, [sp]
	STR r4, [sp, #4]
	LDR r4, =msg_7
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_8
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp]
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, [sp, #4]
	LDR r5, [sp]
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L2
	LDR r4, =msg_9
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	B L3
L2:
	LDR r4, =msg_10
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
L3:
	ADD sp, sp, #8
	LDR r0, =0
	POP {pc}
	.ltorg
p_print_string:
	PUSH {lr}
	LDR r1, [r0]
	ADD r2, r0, #4
	LDR r0, =msg_11
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_12
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
