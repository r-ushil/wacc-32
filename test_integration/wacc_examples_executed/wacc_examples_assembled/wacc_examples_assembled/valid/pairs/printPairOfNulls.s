.data

msg_0:
	.word 4
	.ascii	" = ("
msg_1:
	.word 1
	.ascii	","
msg_2:
	.word 1
	.ascii	")"
msg_3:
	.word 3
	.ascii	"%p\0"
msg_4:
	.word 5
	.ascii	"%.*s\0"
msg_5:
	.word 50
	.ascii	"NullReferenceError: dereference a null reference\n\0"
msg_6:
	.word 1
	.ascii	"\0"

.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #12
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, =0
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, =0
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #8]
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_print_reference
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #4]
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_print_reference
	LDR r4, =msg_1
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	BL p_print_reference
	LDR r4, =msg_2
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	ADD sp, sp, #12
	LDR r0, =0
	POP {pc}
	.ltorg
p_print_reference:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_3
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
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
p_check_null_pointer:
	PUSH {lr}
	CMP r0, #0
	LDREQ r0, =msg_5
	BLEQ p_throw_runtime_error
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_6
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
p_throw_runtime_error:
	BL p_print_string
	MOV r0, #-1
	BL exit
