.data

msg_0:
	.word 53
	.ascii	"Printing an array variable gives an address, such as "
msg_1:
	.word 5
	.ascii	"%.*s\0"
msg_2:
	.word 3
	.ascii	"%p\0"
msg_3:
	.word 1
	.ascii	"\0"

.text

.global main
main:
	PUSH {lr}
	SUB sp, sp, #4
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	LDR r0, =16
	BL malloc
	MOV r4, r0
	LDR r5, =1
	STR r5, [r4, #4]
	LDR r5, =2
	STR r5, [r4, #8]
	LDR r5, =3
	STR r5, [r4, #12]
	LDR r5, =3
	STR r5, [r4]
	STR r4, [sp]
	LDR r4, [sp]
	MOV r0, r4
	BL p_print_reference
	BL p_print_ln
	ADD sp, sp, #4
	LDR r0, =0
	POP {pc}
	.ltorg
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
p_print_reference:
	PUSH {lr}
	MOV r1, r0
	LDR r0, =msg_2
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_3
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
