# KFS-2

Second milestone of the Kernel From Scratch project.

## Objectives

- [x] Create a GDT (Global Descriptor Table)
    - [x] Kernel Code
    - [x] Kernel Data
    - [x] Kernel Stack
    - [x] User Code
    - [x] User Data
    - [x] User Stack
    - [x] Size must not exceed 10 MB
- [x] Declare GDT to the BIOS
- [x] GDT placed at address 0x00000800

## Boot flow

![Bootflow](/assets/KFS-2.svg)

## Kernel bootable via GRUB

The kernel must be loadable by GRUB using the Multiboot format.

Reference:
- https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#OS-image-format

## ASM bootable base

The entry point relies on a Multiboot header embedded in the binary so that GRUB
can identify and load the kernel correctly.

### Multiboot header structure

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

`dd` defines a double word (32 bits).

`section` is the smallest unit of an object file that can be relocated.

## GDT

On IA-32 and x86-64 architectures, in Protected Mode or Long Mode, Interrupt
Service Routines and memory management are controlled through descriptor tables.
Each descriptor stores information about a single object (a service routine, a
task, a chunk of code or data) the CPU might need at some time.

There are three such tables: the Global Descriptor Table (GDT), the Local
Descriptor Table (LDT), and the Interrupt Descriptor Table (IDT). Each is
defined to the CPU via its size and linear address using the `LGDT`, `LLDT`, and
`LIDT` instructions. In practice these tables are loaded once at boot time and
edited only when needed.

## Bonus — Shell

A minimalistic debug shell starts automatically after boot.

| Command  | Description              |
|----------|--------------------------|
| `help`   | List available commands  |
| `stack`  | Print kernel stack dump  |
| `gdt`    | Print GDT layout         |
| `clear`  | Clear the screen         |
| `reboot` | Reboot the system        |
| `halt`   | Halt the CPU             |

### Architecture (SOLID)

| File | Role |
|------|------|
| `src/io.rs` | **SRP** — raw port I/O only (`inb` / `outb`) |
| `src/keyboard.rs` | **DIP** — `KeyboardDriver` trait + `Ps2Keyboard` impl (polling, US layout, shift support) |
| `src/shell.rs` | **OCP** — `Shell<K: KeyboardDriver>` + static `COMMANDS` table; adding a command = one new entry |
| `src/libft.rs` | Auto-scroll, `backspace()`, `reset_cursor()`, VGA hardware cursor (ports 0x3D4 / 0x3D5) |
| `src/gdt.rs` | `pub fn print_info()` used by the `gdt` shell command |

### Scalability for KFS-3

`Shell<K: KeyboardDriver>` uses static dispatch. In KFS-3, implement an
interrupt-driven `IrqKeyboard` that satisfies `KeyboardDriver` and pass it to
the shell — no other code changes required.

## Connect to localhost

```
vncviewer localhost::5900
```
