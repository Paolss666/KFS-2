#![no_std]
use core::panic::PanicInfo;
use crate::types::Colors;
use libft::PrintkArg::*;

pub mod libft;
pub mod types;
pub mod gdt;
pub mod stack;
pub mod io;
pub mod keyboard;
pub mod shell;
pub mod mem;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    libft::printk(b"\n[KERNEL PANIC]\n", Colors::LightRed, &[]);
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main(_magic: u32, _flags: u32) -> ! {
    gdt::init();

    libft::clear_screen();
    libft::printk(b"KFS-2  --  GDT & Stack\n", Colors::LightGreen, &[]);
    libft::printk(b"GDT installed at %x\n", Colors::LightCyan, &[Hex(0x00000800)]);

    stack::print_kernel_stack(32);

    libft::printk(b"\nKeyboard ready. Type 'help' for commands.\n\n",
                  Colors::Yellow, &[]);

    let keyboard = keyboard::Ps2Keyboard::new();
    keyboard.flush();

    let mut sh = shell::Shell::new(keyboard);
    sh.run()
}
