# KFS-2

2eme jalon du projet Kernel From Scratch.

## Objectifs

- [] Create a GDT ( Global Descriptor Table)
    - [] Kernel Code 
    - [] Kernel Data
    - [] Kernel stack 
    - [] User Code
    - [] User data
    - [] User stack
    - [] do not exceed 10MB
- [] Declare GDT to the BIOS 
- [] GDT set in address 0x00000800

## Boot flow

![Bootflow](/assets/KFS-2.svg)

## Kernel bootable via GRUB

Le kernel doit être chargeable par GRUB en respectant le format Multiboot.

Documentation utile :
- https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#OS-image-format

## ASM bootable base

Le point de départ repose sur un header Multiboot placé dans le binaire afin que GRUB reconnaisse correctement le kernel.

### Structure du header Multiboot

```text
┌─────────────────────────────────────────────────────┐
│             MULTIBOOT HEADER (12 bytes)             │
├──────────────┬──────────────────────────────────────┤
│  0x1BADB002  │ MAGIC    -> signature for GRUB       │
│  0x00000003  │ FLAGS    -> requested GRUB features  │
│ -0x1BADB005  │ CHECKSUM -> integrity check          │
└──────────────┴──────────────────────────────────────┘

MAGIC + FLAGS + CHECKSUM = 0
```

dd defines a double word (long word, 32 bits) 


section -> the smallest unit of an object file that can be relocated 



## connect to localhost 
vncviewer localhost::5900
