; Mooltiboot entry point


MBALIGN equ 1 << 0    ; 0000 0001
MEMINFO equ 1 << 1    ; 0000 0010
FLAGS   equ MBALIGN | MEMINFO ; sum 

MAGIX   equ 0x1BADB002 

CHECKSUM equ - (MAGIX + FLAGS)


section .multiboot
align 4
    dd MAGIX             ; offsett 0x0
    dd FLAGS             ; offsett 0x4
    dd CHECKSUM          ; offsett 0x8

section .bss
align 16
stack_bottom:
    resb 16384
stack_top:                 ; the stack growing top -> bottom 


section .text           
global  _start
extern kernel_main        ; this is our main fn in 


_start: 
    mov esp, stack_top       ; asp stack pointer at the top always
    push ebx                 ; pointer to the multiboot info struct -> RAM DISPO 
    push eax                ; MAGIC NUMBER - confirm the download of the GRUB
    call kernel_main


.hang:                      ; SECURITY LOOP 
    cli                     ; Clear Interrupt FLAG unable interrupt
    hlt                     ; Halt pause the CPU
    jmp .hang