
global _start

section .data
str: db 'Hello world!'               ; define byte string
str_len: equ $ - str

section .text
bits 32
_start:
	call put_str                     ; Call Procedure
	mov dword [0xb8f9c], 0x00320034  ; Print '42' to screen (bottom right)
	hlt                              ; Halt

put_str:                             ; → Max string length = 80x25 bytes (VGA buffer size / 2)
	mov ecx, str_len                 ; Initialize count
	jecxz put_str_end                ; Jump if count is 0
	dec ecx                          ; Decrement by 1
	jecxz put_str_first_char         ; Jump if count is 0 (avoid 2³² loop iterations)
put_str_loop:
	mov byte dl, [str + ecx]         ; Move data to register from memory
	mov byte [0xb8000 + ecx * 2], dl ; Move data to memory from register
	loop put_str_loop                ; Decrement count & jump if count ≠ 0
put_str_first_char:
	mov byte dl, [str]
	mov byte [0xb8000], dl
put_str_end:
	ret                              ; Return from Procedure

