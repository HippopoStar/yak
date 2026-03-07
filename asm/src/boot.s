
; Intel 80386 Programmer's Reference Manual
; - 2.4 Instruction Format
; - 2.5.3.2 Effective-Address Computation
; - 3.5.2.3 Executing a Loop or Repeat Zero Times

; NASM - The Netwide Assembler
; - 3.2.2 RESB and Friends: Declaring Uninitialized Data
; - 3.2.4 EQU: Defining Constants
; - 3.9 Local Labels
; - 8.1 BITS: Target Processor Mode
; - 8.4 ABSOLUTE: Defining Absolute Labels
; - A.1 Warning Classes

bits 32
[warning -reloc-abs-dword]                    ; 32-bit absolute section-crossing relocation
[warning -reloc-rel-dword]                    ; 32-bit relative section-crossing relocation

global _start
extern rust_main

absolute 0x000b8000                           ; VGA memory-mapped I/O
	screen_1 resb 0x0fa0
	screen_2 resb 0x0fa0
	screen_3 resb 0x0fa0
	screen_4 resb 0x0fa0
	screen_5 resb 0x0fa0
	screen_6 resb 0x0fa0
	screen_7 resb 0x0fa0
	screen_8 resb 0x0fa0

section .rodata
str: db 'Hello world!'                        ; define byte string
str_len: equ $ - str

section .text
_start:
	mov esp, stack_top
	call put_str                              ; Call Procedure
	mov dword [screen_1 + 0x0f9c], 0x00320034 ; Print '42' to screen (bottom right)
	push 42;
	call rust_main
	hlt                                       ; Halt

put_str:                                      ; -> Max string length = 80x25 bytes (VGA buffer size / 2)
	mov ecx, str_len                          ; Initialize count
	jecxz .end                                ; Jump if count is 0
.loop:
	mov byte dl, [str + (ecx - 1)]            ; Move data to register from memory
	mov byte [screen_1 + (ecx - 1) * 2], dl   ; Move data to memory from register
	loop .loop                                ; Decrement count & jump if count != 0
.end:
	ret                                       ; Return from Procedure

section .bss
stack_bottom:
	resb 4096*4
stack_top:
