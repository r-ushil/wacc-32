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
	SUB sp, sp, #4
	LDR r4, =1
	LDR r5, =2
	LDR r6, =3
	LDR r7, =4
	LDR r8, =5
	LDR r9, =6
	LDR r10, =7
	PUSH {r10}
	LDR r10, =8
	PUSH {r10}
	LDR r10, =9
	PUSH {r10}
	LDR r10, =10
	PUSH {r10}
	LDR r10, =11
	PUSH {r10}
	LDR r10, =12
	PUSH {r10}
	LDR r10, =13
	PUSH {r10}
	LDR r10, =14
	PUSH {r10}
	LDR r10, =15
	PUSH {r10}
	LDR r10, =16
	PUSH {r10}
	LDR r10, =17
	POP {r11}
	ADDS r10, r11, r10
	BLVS p_throw_overflow_error
	POP {r11}
	ADDS r10, r11, r10
	BLVS p_throw_overflow_error
	POP {r11}
	ADDS r10, r11, r10
	BLVS p_throw_overflow_error
	POP {r11}
	ADDS r10, r11, r10
	BLVS p_throw_overflow_error
	POP {r11}
	ADDS r10, r11, r10
	BLVS p_throw_overflow_error
	POP {r11}
	ADDS r10, r11, r10
	BLVS p_throw_overflow_error
	POP {r11}
	ADDS r10, r11, r10
	BLVS p_throw_overflow_error
	POP {r11}
	ADDS r10, r11, r10
	BLVS p_throw_overflow_error
	POP {r11}
	ADDS r10, r11, r10
	BLVS p_throw_overflow_error
	POP {r11}
	ADDS r10, r11, r10
	BLVS p_throw_overflow_error
	ADDS r9, r9, r10
	BLVS p_throw_overflow_error
	ADDS r8, r8, r9
	BLVS p_throw_overflow_error
	ADDS r7, r7, r8
	BLVS p_throw_overflow_error
	ADDS r6, r6, r7
	BLVS p_throw_overflow_error
	ADDS r5, r5, r6
	BLVS p_throw_overflow_error
	ADDS r4, r4, r5
	BLVS p_throw_overflow_error
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	BL exit
	ADD sp, sp, #4
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
