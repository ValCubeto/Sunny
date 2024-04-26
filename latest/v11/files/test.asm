; Constants
section .data
  ; Define a set of bytes. `10` is the ascii code for `\n`
  hello: db "Hello, world!", 10
  ; `$` is the current address. We substract the addres
  ; where `hello` is located to the current address,
  ; getting the length of the string.
  ; This can be manually defined, but be careful
  hello_len: equ $ - hello

; Variables
section .bss
  ; Reserve 256 bytes
  buffer resb 256

; Actual code
section .text
  ; You can change the name of '_start' using the '-o' argument
  global _start
  _start:
    ; Set the system call number to sys_write
    mov rax, 1
    ; Set the fd to stdout
    mov rdi, 1
    ; The text to write
    mov rsi, hello
    ; Text length
    mov rdx, hello_len
    ; Call the kernel
    syscall

    ; 60 = sys_exit
    mov rax, 60
    ; Set exit code to zero
    mov rdi, 0
    ; Call the kernel
    syscall
