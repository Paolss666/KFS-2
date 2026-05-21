use core::arch::asm;
use crate::libft::{printk, printk_char, printk_hex};
use crate::libft::PrintkArg::*;
use crate::types::Colors;

pub fn print_kernel_stack(dwords: usize) {
    let esp: u32;
    let ebp: u32;
    unsafe {
        asm!("mov {}, esp", out(reg) esp);
        asm!("mov {}, ebp", out(reg) ebp);
    }

    printk(b"\n--- Kernel stack dump ---\n", Colors::Yellow, &[]);
    printk(b"ESP = %x   EBP = %x\n", Colors::White,
        &[Hex(esp), Hex(ebp)]);

    let base = esp as *const u32;
    for i in 0..dwords {
        if i % 4 == 0 {
            let addr = esp + (i as u32) * 4;
            printk_hex(addr, Colors::Cyan);
            printk(b":  ", Colors::Cyan, &[]);
        }

        let val = unsafe { core::ptr::read_volatile(base.add(i)) };
        printk_hex(val, Colors::White);
        printk_char(b' ', Colors::White);

        if i % 4 == 3 {
            printk_char(b'\n', Colors::White);
        }
    }
    if dwords % 4 != 0 {
        printk_char(b'\n', Colors::White);
    }
}
