
; Intel 80386 Programmer's Reference Manual
; - 2.4 Instruction Format
; - 2.5.3.2 Effective-Address Computation
; - 3.5.2.3 Executing a Loop or Repeat Zero Times

; NASM - The Netwide Assembler
; - 3.2.2 RESB and Friends: Declaring Uninitialized Data
; - 3.2.4 EQU: Defining Constants
; - 3.9 Local Labels
; - 7.1 BITS: Target Processor Mode

global _start
extern rust_main

section .data
str: db 'Hello world!'                     ; define byte string
str_len: equ $ - str

section .text
bits 32
_start:
	mov esp, stack_top
	call put_str                           ; Call Procedure
	mov dword [0xb8f9c], 0x00320034        ; Print '42' to screen (bottom right)
	call rust_main
	hlt                                    ; Halt

put_str:                                   ; -> Max string length = 80x25 bytes (VGA buffer size / 2)
	mov ecx, str_len                       ; Initialize count
	jecxz .end                             ; Jump if count is 0
.loop:
	mov byte dl, [str + (ecx - 1)]         ; Move data to register from memory
	mov byte [0xb8000 + (ecx - 1) * 2], dl ; Move data to memory from register
	loop .loop                             ; Decrement count & jump if count != 0
.end:
	ret                                    ; Return from Procedure

section .bss
stack_bottom:
	resb 64
stack_top:

