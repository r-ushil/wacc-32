.data

msg_0:
	.word 47
	.ascii	"Please enter the number of integers to insert: "
msg_1:
	.word 10
	.ascii	"There are "
msg_2:
	.word 10
	.ascii	" integers."
msg_3:
	.word 36
	.ascii	"Please enter the number at position "
msg_4:
	.word 3
	.ascii	" : "
msg_5:
	.word 29
	.ascii	"Here are the numbers sorted: "
msg_6:
	.word 0
	.ascii	""
msg_7:
	.word 50
	.ascii	"NullReferenceError: dereference a null reference\n\0"
msg_8:
	.word 3
	.ascii	"%d\0"
msg_9:
	.word 5
	.ascii	"%.*s\0"
msg_10:
	.word 3
	.ascii	"%d\0"
msg_11:
	.word 1
	.ascii	"\0"
msg_12:
	.word 83
	.ascii	"OverflowError: the result is too small/large to store in a 4-byte signed-integer.\n\0"

.text

.global main
f_createNewNode:
	PUSH {lr}
	SUB sp, sp, #8
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #16]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, [sp, #20]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp, #4]
	LDR r0, =8
	BL malloc
	MOV r4, r0
	LDR r5, [sp, #12]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4]
	LDR r5, [sp, #4]
	LDR r0, =4
	BL malloc
	STR r5, [r0]
	STR r0, [r4, #4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	ADD sp, sp, #8
	POP {pc}
	POP {pc}
	.ltorg
f_insert:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, =0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L0
	LDR r4, =0
	STR r4, [sp, #-4]!
	LDR r4, =0
	STR r4, [sp, #-4]!
	LDR r4, [sp, #16]
	STR r4, [sp, #-4]!
	BL f_createNewNode
	ADD sp, sp, #12
	MOV r4, r0
	STR r4, [sp, #4]
	B L1
L0:
	SUB sp, sp, #12
	LDR r4, [sp, #16]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #8]
	LDR r4, [sp, #16]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #4]
	LDR r4, =0
	STR r4, [sp]
	LDR r4, [sp, #20]
	LDR r5, [sp, #4]
	CMP r4, r5
	MOVLT r4, #1
	MOVGE r4, #0
	CMP r4, #0
	BEQ L2
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp, #20]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_insert
	ADD sp, sp, #8
	MOV r4, r0
	LDR r5, [sp, #8]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5]
	STR r4, [r5]
	B L3
L2:
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp, #20]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_insert
	ADD sp, sp, #8
	MOV r4, r0
	LDR r5, [sp, #8]
	MOV r0, r5
	BL p_check_null_pointer
	LDR r5, [r5, #4]
	STR r4, [r5]
L3:
	ADD sp, sp, #12
L1:
	LDR r4, [sp, #4]
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
f_printTree:
	PUSH {lr}
	LDR r4, [sp, #4]
	LDR r5, =0
	CMP r4, r5
	MOVEQ r4, #1
	MOVNE r4, #0
	CMP r4, #0
	BEQ L4
	LDR r4, =0
	MOV r0, r4
	POP {pc}
	B L5
L4:
	SUB sp, sp, #12
	LDR r4, [sp, #16]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #8]
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp, #4]
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_printTree
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp]
	LDR r4, [sp, #16]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4]
	LDR r4, [r4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	BL p_print_int
	MOV r4, #' '
	MOV r0, r4
	BL putchar
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_check_null_pointer
	LDR r4, [r4, #4]
	LDR r4, [r4]
	STR r4, [sp, #4]
	LDR r4, [sp, #4]
	STR r4, [sp, #-4]!
	BL f_printTree
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp]
	LDR r4, =0
	MOV r0, r4
	ADD sp, sp, #12
	POP {pc}
	ADD sp, sp, #12
L5:
	POP {pc}
	.ltorg
main:
	PUSH {lr}
	SUB sp, sp, #12
	LDR r4, =0
	STR r4, [sp, #8]
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	ADD r4, sp, #8
	MOV r0, r4
	BL p_read_int
	LDR r4, =msg_1
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #8]
	MOV r0, r4
	BL p_print_int
	LDR r4, =msg_2
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =0
	STR r4, [sp, #4]
	LDR r4, =0
	STR r4, [sp]
	B L6
L7:
	SUB sp, sp, #4
	LDR r4, =0
	STR r4, [sp]
	LDR r4, =msg_3
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #8]
	LDR r5, =1
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	MOV r0, r4
	BL p_print_int
	LDR r4, =msg_4
	MOV r0, r4
	BL p_print_string
	ADD r4, sp, #0
	MOV r0, r4
	BL p_read_int
	LDR r4, [sp]
	STR r4, [sp, #-4]!
	LDR r4, [sp, #8]
	STR r4, [sp, #-4]!
	BL f_insert
	ADD sp, sp, #8
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, [sp, #8]
	LDR r5, =1
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #8]
	ADD sp, sp, #4
L6:
	LDR r4, [sp, #4]
	LDR r5, [sp, #8]
	CMP r4, r5
	MOVLT r4, #1
	MOVGE r4, #0
	CMP r4, #1
	BEQ L7
	LDR r4, =msg_5
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp]
	STR r4, [sp, #-4]!
	BL f_printTree
	ADD sp, sp, #4
	MOV r4, r0
	STR r4, [sp, #4]
	LDR r4, =msg_6
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	ADD sp, sp, #12
	LDR r0, =0
	POP {pc}
	.ltorg
p_check_null_pointer:
	PUSH {lr}
	CMP r0, #0
	LDREQ r0, =msg_7
	BLEQ p_throw_runtime_error
	POP {pc}
p_print_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_8
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_string:
	PUSH {lr}
	LDR r1, [r0]
	ADD r2, r0, #4
	LDR r0, =msg_9
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_read_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_10
	ADD r0, r0, #4
	BL scanf
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_11
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
p_throw_overflow_error:
	LDR r0, =msg_12
	BL p_throw_runtime_error
p_throw_runtime_error:
	BL p_print_string
	MOV r0, #-1
	BL exit
