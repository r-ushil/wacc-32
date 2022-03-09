.data

msg_0:
	.word 50
	.ascii	"NullReferenceError: dereference a null reference\n\0"
msg_1:
	.word 1
	.ascii	"\0"
msg_2:
	.word 5
	.ascii	"%.*s\0"

.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #5
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, =10
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	MOV r5, #'a'
	LDR r0, =1
	BL malloc
	STRB r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #1]
	LDR r4, [sp, #1]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp]
	LDRSB r4, [sp]
	MOV r0, r4
	BL putchar
	BL p_print_ln
	MOV r4, #'Z'
	LDR r5, [sp, #1]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STRB r4, [r5]
	LDR r4, [sp, #1]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDRSB r4, [r4]
	STRB r4, [sp]
	LDRSB r4, [sp]
	MOV r0, r4
	BL putchar
	BL p_print_ln
	ADD sp, sp, #5
	LDR r0, =0
	POP {pc}
	.ltorg
p_check_null_pointer:
	PUSH {lr}
	CMP r0, #0
	LDREQ r0, =msg_0
	BLEQ p_throw_runtime_error
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_1
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
p_throw_runtime_error:
	BL p_print_string
	MOV r0, #-1
	BL exit
p_print_string:
	PUSH {lr}
	LDR r1, [r0]
	ADD r2, r0, #4
	LDR r0, =msg_2
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
