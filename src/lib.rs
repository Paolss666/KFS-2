#![no_std]
use core::panic::PanicInfo;
use crate::types::Colors;
use libft::PrintkArg::*;

pub mod libft;
pub mod types;
pub mod gdt;
pub mod stack;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    
    loop {}

}

#[no_mangle]
pub extern "C" fn kernel_main(_magic: u32, _flags: u32) -> ! {
    gdt::init();

    libft::clear_screen();
    libft::printk(b"KFS-2 booted\n", Colors::LightGreen, &[]);
    libft::printk(b"GDT installed at %x\n", Colors::LightCyan,
        &[Hex(0x00000800)]);

    stack::print_kernel_stack(32);

    loop {}
}
