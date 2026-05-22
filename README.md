# KFS-2

2eme jalon du projet Kernel From Scratch.

## Objectifs

- [x] Create a GDT ( Global Descriptor Table)
    - [x] Kernel Code 
    - [x] Kernel Data
    - [x] Kernel stack 
    - [x] User Code
    - [x] User data
    - [x] User stack
    - [x] do not exceed 10MB
- [x] Declare GDT to the BIOS 
- [x] GDT set in address 0x00000800

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

## GDT
On the IA-32 and x86-64 architectures, and more precisely in Protected Mode or Long Mode, Interrupt Service Routines and a good deal of memory management are controlled through tables of descriptors. Each descriptor stores information about a single object (e.g. a service routine, a task, a chunk of code or data) the CPU might need at some time. If you try, for instance, to load a new value into a Segment Register, the CPU needs to perform safety and access control checks to see whether you're actually entitled to access that specific memory area. Once the checks are performed, useful values (such as the lowest and highest addresses) are cached in invisible CPU registers.

On these architectures, there are three of this type of table: The Global Descriptor Table, the Local Descriptor Table and the Interrupt Descriptor Table (which supplants the Interrupt Vector Table). Each table is defined using their size and linear address to the CPU through the LGDT, LLDT, and LIDT instructions respectively. In almost all use cases, these tables are only placed into memory once, at boot time, and then edited later when needed.  


## connect to localhost 
vncviewer localhost::5900
