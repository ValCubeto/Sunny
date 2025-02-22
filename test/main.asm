;; Flat Assembler

;; Compiler stuff
format ELF64 executable
entry _start
struct fix struc



struct String [data] {
  common
  . db data
  .len = $ - .
}

struct ReservedString [size] {
  common
  . rb size
  .len = size
}

macro defstr name, [data] {
  name String data
}



; struct Stat {
;   ; todo!
;   .st_dev rb 1
; }

;; Compile-time constants
;;;; sys_read(fd: uint32 = rdi, buf: &char = rsi, count: usize = rdx)
SYS_READ  equ 0
;;;; sys_write(fd: uint32 = rdi, buf: &char = rsi, count: usize = rdx)
SYS_WRITE equ 1
;;;; sys_open(pathname: &char = rdi, flags: int32 = rsi, mode: int32 = rdx)
SYS_OPEN  equ 2
;;;; sys_close(fd: int32 = rdi)
SYS_CLOSE equ 3
;;;; sys_stat(pathname: &char = rdi, stat: &Stat = rsi)
SYS_STAT  equ 4
;;;; sys_fstat(fd: int32 = rdi, stat: &Stat = rsi)
SYS_FSTAT equ 5
;;;; sys_lstat(pathname: &char = rdi, stat: &Stat = rsi)
SYS_LSTAT equ 6
;;;; sys_get_pid()
SYS_GET_PID equ 32
;;;; sys_exit(fd: int32 = rdi)
SYS_EXIT  equ 60

STDIN  equ 0
STDOUT equ 1
STDERR equ 2

;; Macros
macro print bytes*, len* {
  mov rax, SYS_WRITE
  mov rdi, STDOUT
  ; lea rsi, [bytes]
  mov rsi, bytes
  mov rdx, len
  syscall
}

macro eprint bytes*, len* {
  mov rax, SYS_WRITE
  mov rdi, STDERR
  mov rsi, bytes
  mov rdx, len
  syscall
}

macro println bytes*, len* {
  ;; TODO: alloc len + 1 bytes (for the new line), copy `bytes` into it, and print it
  print bytes, len
  print EOL, 1
}

macro get_pid pid {
  mov rax, SYS_GET_PID
  syscall
  mov [pid], rax
}

macro exit code* {
  mov rax, SYS_EXIT
  mov rdi, code
  syscall
}

; macro for i*, start*, end*, step = 1, code* {
;   local .loop_start, .loop_end
;   mov [i] start
;   .loop_start:
;     cmp [i], end
;     code
;     jge .loop_end
;     add [i], step
;     jmp .loop_start
;   .loop_end:
; }

int2str:
  local .loop_start
  mov rax, int
  mov rdi, str + str.len  ;; index
  mov rcx, 10  ;; divide by 10
  .loop_start:
  dec rdi
  xor rdx, rdx  ;; clean
  div rcx
  add dl, '0'
  mov [rdi] dl
  test rax, rax
  jnz .loop_start

  mov rsi, rdi
  mov rdx, str + str.len
  sub rdx, rsi
  retn 4 * 3  ;; return and clean 3 params


defstr EOL, 10
; defstr hello, "Hello, world!"
defstr pid_text, "PID: "

pid rq 1
pid_str ReservedString 20

_start:
  ; println hello, hello.len
  get_pid pid
  push pid_str.len
  push pid_str
  push pid
  call int2str
  print pid_text
  println pid_str, pid_str.len
  exit 0
