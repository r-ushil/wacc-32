.data

msg_0:
	.word 1
	.ascii	"-"
msg_1:
	.word 0
	.ascii	""
msg_2:
	.word 3
	.ascii	"|  "
msg_3:
	.word 1
	.ascii	" "
msg_4:
	.word 3
	.ascii	" = "
msg_5:
	.word 3
	.ascii	"  |"
msg_6:
	.word 29
	.ascii	"Ascii character lookup table:"
msg_7:
	.word 5
	.ascii	"%.*s\0"
msg_8:
	.word 83
	.ascii	"OverflowError: the result is too small/large to store in a 4-byte signed-integer.\n\0"
msg_9:
	.word 1
	.ascii	"\0"
msg_10:
	.word 3
	.ascii	"%d\0"

.text

.global main
f_printLine:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, =0
	STR r4, [sp]
	B L0
L1:
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp]
	LDR r5, =1
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp]
L0:
	LDR r4, [sp]
	LDR r5, [sp, #8]
	CMP r4, r5
	MOVLT r4, #1
	MOVGE r4, #0
	CMP r4, #1
	BEQ L1
	LDR r4, =msg_1
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	MOV r4, #1
	MOV r0, r4
	ADD sp, sp, #4
	POP {pc}
	POP {pc}
	.ltorg
f_printMap:
	PUSH {lr}
	LDR r4, =msg_2
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #4]
	LDR r5, =100
	CMP r4, r5
	MOVLT r4, #1
	MOVGE r4, #0
	CMP r4, #0
	BEQ L2
	LDR r4, =msg_3
	MOV r0, r4
	BL p_print_string
	B L3
L2:
L3:
	LDR r4, [sp, #4]
	MOV r0, r4
	BL p_print_int
	LDR r4, =msg_4
	MOV r0, r4
	BL p_print_string
	LDR r4, [sp, #4]
	MOV r0, r4
	BL putchar
	LDR r4, =msg_5
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	MOV r4, #1
	MOV r0, r4
	POP {pc}
	POP {pc}
	.ltorg
main:
	PUSH {lr}
	SUB sp, sp, #5
	LDR r4, =msg_6
	MOV r0, r4
	BL p_print_string
	BL p_print_ln
	LDR r4, =13
	STR r4, [sp, #-4]!
	BL f_printLine
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp, #4]
	MOV r4, #' '
	STR r4, [sp]
	B L4
L5:
	LDR r4, [sp]
	STR r4, [sp, #-4]!
	BL f_printMap
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp, #4]
	LDR r4, [sp]
	LDR r5, =1
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp]
L4:
	LDR r4, [sp]
	LDR r5, =127
	CMP r4, r5
	MOVLT r4, #1
	MOVGE r4, #0
	CMP r4, #1
	BEQ L5
	LDR r4, =13
	STR r4, [sp, #-4]!
	BL f_printLine
	ADD sp, sp, #4
	MOV r4, r0
	STRB r4, [sp, #4]
	ADD sp, sp, #5
	LDR r0, =0
	POP {pc}
	.ltorg
p_print_string:
	PUSH {lr}
	LDR r1, [r0]
	ADD r2, r0, #4
	LDR r0, =msg_7
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_throw_overflow_error:
	LDR r0, =msg_8
	BL p_throw_runtime_error
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_9
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_int:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_10
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_throw_runtime_error:
	BL p_print_string
	MOV r0, #-1
	BL exit
