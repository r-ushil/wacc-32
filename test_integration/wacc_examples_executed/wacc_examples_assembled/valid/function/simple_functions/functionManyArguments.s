.data

msg_0:
	.word 5
	.ascii	"a is "
msg_1:
	.word 5
	.ascii	"b is "
msg_2:
	.word 5
	.ascii	"c is "
msg_3:
	.word 5
	.ascii	"d is "
msg_4:
	.word 5
	.ascii	"e is "
msg_5:
	.word 5
	.ascii	"f is "
msg_6:
	.word 5
	.ascii	"hello"
msg_7:
	.word 10
	.ascii	"answer is "
msg_8:
	.word 5
	.ascii	"%.*s\0"
msg_9:
	.word 3
	.ascii	"%d\0"
msg_10:
	.word 1
	.ascii	"\0"
msg_11:
	.word 5
	.ascii	"true\0"
msg_12:
	.word 6
	.ascii	"false\0"
msg_13:
	.word 3
	.ascii	"%p\0"

.text

.global main
f_doSomething:
	PUSH {lr}
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	LDR r4, =msg_1
	MOV r0, r4
	BL p_print_string
	LDRSB r4, [sp, #8]
	MOV r0, r4
	BL p_print_bool
	BL p_print_ln
	LDR r4, =msg_2
	MOV r0, r4
	BL p_print_string
	LDRSB r4, [sp, #9]
	MOV r0, r4
	BL putchar
	BL p_print_ln
	LDR r4, =msg_3
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #10]
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =msg_4
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #14]
	MOV r0, r4
	BL p_print_reference
	BL p_print_ln
	LDR r4, =msg_5
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #18]
	MOV r0, r4
	BL p_print_reference
	BL p_print_ln
	MOV r4, #'g'
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
main:
	PUSH {lr}
	SUB sp, sp, #9
	LDR r0, =6
	BL malloc
	MOV r4, r0
	MOV r5, #0
	STRB r5, [r4, #4]
	MOV r5, #1
	STRB r5, [r4, #5]
	LDR r5, =2
	STR r5, [r4]
	STR r4, [sp, #5]
	LDR r0, =12
	BL malloc
	MOV r4, r0
	LDR r5, =1
	STR r5, [r4, #4]
	LDR r5, =2
	STR r5, [r4, #8]
	LDR r5, =2
	STR r5, [r4]
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #9]
	STR r4, [sp, #-4]!
	LDR r4, =msg_6
	STR r4, [sp, #-4]!
	MOV r4, #'u'
	STRB r4, [sp, #-1]!
	MOV r4, #1
	STRB r4, [sp, #-1]!
	LDR r4, =42
	STR r4, [sp, #-4]!
	BL f_doSomething
	ADD sp, sp, #18
	MOV r4, r0
	STRB r4, [sp]
	LDR r4, =msg_7
	MOV r0, r4
	BL p_print_string
	LDRSB r4, [sp]
	MOV r0, r4
	BL putchar
	BL p_print_ln
	ADD sp, sp, #9
	LDR r0, =0
	POP {pc}
	.ltorg
p_print_string:
	PUSH {lr}
	LDR r1, [r0]
	ADD r2, r0, #4
	LDR r0, =msg_8
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_9
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_10
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_bool:
	PUSH {lr}
	CMP r0, #0
	LDRNE r0, =msg_11
	LDREQ r0, =msg_12
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_reference:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_13
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
