.data

msg_0:
	.word 8
	.ascii	"True is "
msg_1:
	.word 9
	.ascii	"False is "
msg_2:
	.word 5
	.ascii	"%.*s\0"
msg_3:
	.word 5
	.ascii	"true\0"
msg_4:
	.word 6
	.ascii	"false\0"
msg_5:
	.word 1
	.ascii	"\0"

.text

.global main
main:
	PUSH {lr}
	LDR r4, =msg_0
	MOV r0, r4
	BL p_print_string
	MOV r4, #1
	MOV r0, r4
	BL p_print_bool
	BL p_print_ln
	LDR r4, =msg_1
	MOV r0, r4
	BL p_print_string
	MOV r4, #0
	MOV r0, r4
	BL p_print_bool
	BL p_print_ln
	LDR r0, =0
	POP {pc}
	.ltorg
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
p_print_bool:
	PUSH {lr}
	CMP r0, #0
	LDRNE r0, =msg_3
	LDREQ r0, =msg_4
	ADD r0, r0, #4
	BL printf
	MOV r0, #0
	BL fflush
	POP {pc}
p_print_ln:
	PUSH {lr}
	LDR r0, =msg_5
	ADD r0, r0, #4
	BL puts
	MOV r0, #0
	BL fflush
	POP {pc}
