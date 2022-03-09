.data

msg_0:
	.word 83
	.ascii	"OverflowError: the result is too small/large to store in a 4-byte signed-integer.\n\0"
msg_1:
	.word 5
	.ascii	"%.*s\0"

.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #36
	LDR r4, =1
	LDR r5, =2
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #32]
	LDR r4, =3
	LDR r5, =4
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #28]
	LDR r4, =5
	LDR r5, =6
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #24]
	LDR r4, =7
	LDR r5, =8
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #20]
	LDR r4, =9
	LDR r5, =10
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #16]
	LDR r4, =11
	LDR r5, =12
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #12]
	LDR r4, =13
	LDR r5, =14
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #8]
	LDR r4, =15
	LDR r5, =16
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp, #4]
	LDR r4, =17
	STR r4, [sp]
	LDR r4, [sp, #32]
	LDR r5, [sp, #28]
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, [sp, #24]
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, [sp, #20]
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, [sp, #16]
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, [sp, #12]
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, [sp, #8]
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, [sp, #4]
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	LDR r5, [sp]
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	MOV r0, r4
	BL exit
	ADD sp, sp, #36
	LDR r0, =0
	POP {pc}
	.ltorg
p_throw_overflow_error:
	LDR r0, =msg_0
	BL p_throw_runtime_error
p_throw_runtime_error:
	BL p_print_string
	MOV r0, #-1
	BL exit
p_print_string:
	PUSH {lr}
	LDR r1, [r0]
	ADD r2, r0, #4
	LDR r0, =msg_1
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
