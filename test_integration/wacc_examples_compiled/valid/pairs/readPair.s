.data

msg_0:
	.word 39
	.ascii	"Please enter the first element (char): "
msg_1:
	.word 39
	.ascii	"Please enter the second element (int): "
msg_2:
	.word 22
	.ascii	"The first element was "
msg_3:
	.word 23
	.ascii	"The second element was "
msg_4:
	.word 5
	.ascii	"%.*s\0"
msg_5:
	.word 4
	.ascii	" %c\0"
msg_6:
	.word 50
	.ascii	"NullReferenceError: dereference a null reference\n\0"
msg_7:
	.word 3
	.ascii	"%d\0"
msg_8:
	.word 1
	.ascii	"\0"
msg_9:
	.word 3
	.ascii	"%d\0"

.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #9
	LDR r0, =8
	BL malloc
	MOV r4, r0
	MOV r5, #0
	LDR r0, =1
	BL malloc
	STRB r5, [r0]
	STR r0, [r4]
	LDR r5, =0
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #5]
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	MOV r4, #'0'
	STRB r4, [sp, #4]
	ADD r4, sp, #4
	MOV r0, r4
	BL p_read_char
	LDRSB r4, [sp, #4]
	LDR r5, [sp, #5]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5]
	STRB r4, [r5]
	LDR r4, =msg_1
	MOV r0, r4
	BL p_print_string
	LDR r4, =0
	STR r4, [sp]
	ADD r4, sp, #0
	MOV r0, r4
	BL p_read_int
	LDR r4, [sp]
	LDR r5, [sp, #5]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STR r4, [r5]
	MOV r4, #0
	STRB r4, [sp, #4]
	LDR r4, =-1
	STR r4, [sp]
	LDR r4, =msg_2
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #5]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDRSB r4, [r4]
	STRB r4, [sp, #4]
	LDRSB r4, [sp, #4]
	MOV r0, r4
	BL putchar
	BL p_print_ln
	LDR r4, =msg_3
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #5]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	BL p_print_int
	BL p_print_ln
	ADD sp, sp, #9
	LDR r0, =0
	POP {pc}
	.ltorg
p_print_string:
	PUSH {lr}
	LDR r1, [r0]
	ADD r2, r0, #4
	LDR r0, =msg_4
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_read_char:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_5
	ADD r0, r0, #4
	BL scanf
	POP {pc}
p_check_null_pointer:
	PUSH {lr}
	CMP r0, #0
	LDREQ r0, =msg_6
	BLEQ p_throw_runtime_error
	POP {pc}
p_read_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_7
	ADD r0, r0, #4
	BL scanf
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_8
	ADD r0, r0, #4
	BL puts
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
p_throw_runtime_error:
	BL p_print_string
	MOV r0, #-1
	BL exit
