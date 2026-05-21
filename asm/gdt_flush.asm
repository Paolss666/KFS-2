global gdt_flush

section .note.GNU-stack noalloc noexec nowrite progbits

section .text
gdt_flush:
    mov eax, [esp + 4]      ; arg1 = pointer to GDTR struct
    lgdt [eax]              ; load GDT into CPU register

    mov ax, 0x10            ; kernel data selector
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov ax, 0x18            ; kernel stack selector
    mov ss, ax

    jmp 0x08:.flush         ; far jump reloads CS with kernel code selector
.flush:
    ret
