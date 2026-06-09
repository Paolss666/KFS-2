use crate::keyboard::KeyboardDriver;
use crate::libft::{printk, printk_char, printk_str, backspace, reset_cursor, clear_screen};
use crate::types::Colors;

// ── Command table 

pub enum Flow {
    Continue,
    Exit,
}

pub struct Command {
    pub name:        &'static [u8],
    pub description: &'static [u8],
    pub run:         fn() -> Flow,
}

static COMMANDS: &[Command] = &[
    Command { name: b"help",   description: b"List available commands",     run: cmd_help   },
    Command { name: b"stack",  description: b"Print kernel stack dump",     run: cmd_stack  },
    Command { name: b"gdt",    description: b"Print GDT layout",            run: cmd_gdt    },
    Command { name: b"clear",  description: b"Clear the screen",            run: cmd_clear  },
    Command { name: b"reboot", description: b"Reboot the system",           run: cmd_reboot },
    Command { name: b"halt",   description: b"Halt the CPU",                run: cmd_halt   },
    Command { name: b"exit",   description: b"Cause the shell to exit",     run: cmd_exit   },
];

// ── Command implementations ───────────────────────────────────────────────

fn cmd_help() -> Flow {
    printk(b"\nAvailable commands:\n", Colors::LightCyan, &[]);
    for cmd in COMMANDS {
        printk(b"  ", Colors::White, &[]);
        printk_str(cmd.name, Colors::Yellow);
        // right-pad name to 10 chars
        let name_len = crate::libft::strlen(cmd.name);
        let mut pad = name_len;
        while pad < 10 { printk_char(b' ', Colors::White); pad += 1; }
        printk_str(cmd.description, Colors::Gray);
        printk_char(b'\n', Colors::White);
    }
    printk_char(b'\n', Colors::White);
    Flow::Continue
}

fn cmd_exit() -> Flow{
    Flow::Exit
}


fn shutdown() -> ! {
    unsafe {
        // QEMU (>= 2.0): ACPI poweroff
        core::arch::asm!(
            "out dx, ax",
            in("dx") 0x604u16,
            in("ax") 0x2000u16,
            options(nomem, nostack, preserves_flags),
        );
    }
    unsafe { core::arch::asm!("cli"); }
    loop { unsafe { core::arch::asm!("hlt"); } }
}


fn cmd_stack() -> Flow {
    crate::stack::print_kernel_stack(32);
    Flow::Continue
}

fn cmd_gdt() -> Flow {
    crate::gdt::print_info();
    Flow::Continue
}

fn cmd_clear() -> Flow {
    clear_screen();
    reset_cursor();
    Flow::Continue
}

fn cmd_reboot() -> Flow {
    printk(b"Rebooting...\n", Colors::LightRed, &[]);
    unsafe {
        // Triple-fault reset: load a null IDT (limit=0) then trigger a
        // divide-by-zero. With no handler the CPU triple-faults and resets.
        // Most reliable method in QEMU / KVM — no ACPI or PS/2 controller needed.
        let idt_null: [u8; 6] = [0; 6];
        core::arch::asm!(
            "lidt [{ptr}]",  // load null IDT
            "xor eax, eax",  // eax = 0
            "div eax",       // #DE -> no handler -> #DF -> no handler -> triple fault -> reset
            ptr = in(reg) idt_null.as_ptr(),
            out("eax") _,
            options(nostack),
        );
    }
    loop {}
}

fn cmd_halt() -> Flow {
    printk(b"Halting...\n", Colors::LightRed, &[]);
    unsafe { core::arch::asm!("cli", options(nomem, nostack)); }
    loop { unsafe { core::arch::asm!("hlt", options(nomem, nostack)); } }
}

// ── Shell REPL ────────────────────────────────────────────────────────────

const INPUT_MAX: usize = 256;
const PROMPT:    &[u8] = b"kfs> ";

/// Minimalistic debug shell.
/// Generic over any `KeyboardDriver` implementation — easy to swap for
/// interrupt-driven input in KFS-3.
pub struct Shell<K: KeyboardDriver> {
    keyboard:  K,
    buf:       [u8; INPUT_MAX],
    len:       usize,
}

impl<K: KeyboardDriver> Shell<K> {
    pub fn new(keyboard: K) -> Self {
        Shell { keyboard, buf: [0u8; INPUT_MAX], len: 0 }
    }

    pub fn run(&mut self) -> ! {
        self.print_prompt();
        loop {
            if let Some(c) = self.keyboard.poll() {
                if let Flow::Exit = self.handle_char(c) {
                    printk(b"\nShell exited. Powering off.\n", Colors::LightRed, &[]);
                    shutdown();
                }
            }
        }
    }

    fn print_prompt(&self) {
        printk(PROMPT, Colors::LightGreen, &[]);
    }

    fn handle_char(&mut self, c: u8) -> Flow {
        match c {
            b'\n' => {
                printk_char(b'\n', Colors::White);
                let flow = self.execute();
                for b in self.buf.iter_mut() { *b = 0; }
                self.len = 0;
                if let Flow::Exit = flow {
                    return Flow::Exit;
                }
                self.print_prompt();
                Flow::Continue
            }
            0x08 => {
                // Backspace
                if self.len > 0 {
                    self.len -= 1;
                    self.buf[self.len] = 0;
                    backspace();
                }
                Flow::Continue
            }
            c if c >= b' ' && c < 0x7F => {
                if self.len < INPUT_MAX - 1 {
                    self.buf[self.len] = c;
                    self.len += 1;
                    printk_char(c, Colors::White);
                }
                Flow::Continue
            }
            _ => Flow::Continue
        }
    }

    fn execute(&self)  -> Flow{
        let input = &self.buf[..self.len];
        if self.len == 0 { return Flow::Continue; }

        for cmd in COMMANDS {
            if bytes_eq(input, cmd.name) {
                return (cmd.run)();
            }
        }

        printk(b"unknown command: '", Colors::LightRed, &[]);
        printk_str(input, Colors::White);
        printk(b"'  (type 'help')\n", Colors::LightRed, &[]);
        Flow::Continue
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────

fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return false; }
    for i in 0..a.len() {
        if a[i] != b[i] { return false; }
    }
    true
}
