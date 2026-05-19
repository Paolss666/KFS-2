#![no_std]
use core::panic::PanicInfo;
use crate::types::Colors;
use libft::PrintkArg::*;

pub mod libft;
pub mod types;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    
    loop {}

}

#[no_mangle]
pub extern "C" fn kernel_main(_magic: u32, _flags: u32) -> ! {
    let str = b"Hello World\0";

    libft::clear_screen();
    libft::printk(b"%s\n", Colors::LightMagenta, &[Str(str)]);
    libft::printk(b"Positive: %d\n", Colors::White, &[Int(42)]);
    libft::printk(b"Negative: %d\n", Colors::White, &[Int(-123)]);
    libft::printk(b"Zero: %d\n", Colors::White, &[Int(0)]);
    libft::printk(b"String: %s\n", Colors::Green, &[Str(b"Hello KFS")]);
    libft::printk(b"%s = %d\n", Colors::Cyan, &[Str(b"answer"), Int(42)]);
    libft::printk(b"100%% done\n", Colors::Red, &[]);
    libft::printk(b"unknown flag: %z\n", Colors::Magenta, &[]);
    libft::printk(b"[%s] count=%d status=%s\n", Colors::LightBlue, &[
        Str(b"INFO"),
        Int(7),
        Str(b"OK"),
    ]);
    libft::printk(b"\n\n",Colors::Black, &[]);
    libft::printk(b"##  ##  ######\n", Colors::Red, &[]);
    libft::printk(b"##  ##      ##\n", Colors::Green, &[]);
    libft::printk(b"######    ##  \n", Colors::Magenta, &[]);
    libft::printk(b"    ##   ##   \n", Colors::LightBlue, &[]);
    libft::printk(b"    ##  ######\n", Colors::Yellow, &[]);

    loop {}
}
