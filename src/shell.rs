use crate::keyboard::KeyboardDriver;
use crate::libft::{printk, printk_char, printk_str, backspace, reset_cursor, clear_screen};
use crate::types::Colors;

// ── Command table 

pub struct Command {
    pub name:        &'static [u8],
    pub description: &'static [u8],
    pub run:         fn(),
}

static COMMANDS: &[Command] = &[
    Command { name: b"help",   description: b"List available commands",     run: cmd_help   },
    Command { name: b"stack",  description: b"Print kernel stack dump",     run: cmd_stack  },
    Command { name: b"gdt",    description: b"Print GDT layout",            run: cmd_gdt    },
    Command { name: b"clear",  description: b"Clear the screen",            run: cmd_clear  },
    Command { name: b"reboot", description: b"Reboot the system",           run: cmd_reboot },
    Command { name: b"halt",   description: b"Halt the CPU",                run: cmd_halt   },
];

// ── Command implementations ───────────────────────────────────────────────

fn cmd_help() {
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
}

fn cmd_stack() {
    crate::stack::print_kernel_stack(32);
}

fn cmd_gdt() {
    crate::gdt::print_info();
}

fn cmd_clear() {
    clear_screen();
    reset_cursor();
}

fn cmd_reboot() {
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

fn cmd_halt() {
    printk(b"Halting...\n", Colors::LightRed, &[]);
    unsafe { core::arch::asm!("cli", options(nomem, nostack));}
    unsafe { core::arch::asm!("hlt", out("eax") _,options(nostack))}
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
                self.handle_char(c);
            }
        }
    }

    fn print_prompt(&self) {
        printk(PROMPT, Colors::LightGreen, &[]);
    }

    fn handle_char(&mut self, c: u8) {
        match c {
            b'\n' => {
                printk_char(b'\n', Colors::White);
                self.execute();
                for b in self.buf.iter_mut() { *b = 0; }
                self.len = 0;
                self.print_prompt();
            }
            0x08 => {
                // Backspace
                if self.len > 0 {
                    self.len -= 1;
                    self.buf[self.len] = 0;
                    backspace();
                }
            }
            c if c >= b' ' && c < 0x7F => {
                if self.len < INPUT_MAX - 1 {
                    self.buf[self.len] = c;
                    self.len += 1;
                    printk_char(c, Colors::White);
                }
            }
            _ => {}
        }
    }

    fn execute(&self) {
        let input = &self.buf[..self.len];
        if self.len == 0 { return; }

        for cmd in COMMANDS {
            if bytes_eq(input, cmd.name) {
                (cmd.run)();
                return;
            }
        }

        printk(b"unknown command: '", Colors::LightRed, &[]);
        printk_str(input, Colors::White);
        printk(b"'  (type 'help')\n", Colors::LightRed, &[]);
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
