
global start

section .data
str: db 'Hello world! (Quit qemu: Alt+2, then type "q" and press Enter)'
str_len: equ $ - str

section .text
bits 32
start:
	call put_str                     ; Call Procedure
	mov dword [0xb8f9c], 0x00320034  ; Print '42' to screen (bottom right)
	hlt                              ; Halt

put_str:                             ; → Max string length = 80x25 bytes (VGA buffer size / 2)
	mov ecx, str_len                 ; Initialize count
	jecxz put_str_end                ; Jump if count is 0
	mov eax, 0                       ; Initialize accumulator
put_str_loop:
	mov byte dl, [str + eax]         ; Move data from memory to register
	mov byte [0xb8000 + eax * 2], dl ; Move data from register to memory
	inc eax                          ; Increment by 1
	loop put_str_loop                ; Decrement count & jump if count ≠ 0
put_str_end:
	ret                              ; Return from Procedure

