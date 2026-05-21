use crate::types::VGA_BUFFER;
use crate::types::Colors;

const VGA_WIDTH: usize = 80;
static mut CURSOR_COL: usize = 0;
static mut CURSOR_ROW: usize = 0;

pub enum PrintkArg {
    Int(i32),
    Str(&'static [u8]),
    Hex(u32),
}


pub fn strlen(s: &[u8]) -> usize {
    let mut len = 0;
    while len < s.len() && s[len] != 0 {
        len+=1;
    }
    len
}

pub fn strcmp(s1: &[u8], s2: &[u8]) -> i32 {
    let mut i = 0;
        loop {
            let c1 = s1[i];
            let c2 = s2[i];
            if c1 != c2 || c1 == 0 {
                return (c1 - c2).into();
            }
            i += 1;
        }
}

pub fn putchar(c: u8, color: Colors, pos: usize) {
    unsafe {
        core::ptr::write_volatile(VGA_BUFFER.add(pos * 2), c);
        core::ptr::write_volatile(VGA_BUFFER.add(pos * 2 + 1), color as u8);
    }
}

pub fn putstr(s: &[u8], color: Colors, start_pos: usize) {
    let mut pos = start_pos;
    for &c in s {
        if c == 0 { break; }
        if c == b'\n' {
            pos = (pos / VGA_WIDTH + 1) * VGA_WIDTH;
        } else {
            putchar(c, color, pos);
            pos += 1;
        }
    }
}


pub fn putnbr(n: isize, color: Colors, pos: usize) -> usize {
    if n < 0 {
        putchar(b'-', color, pos);
        putnbr(-n, color, pos + 1)
    } else if n > 9 {
        let cur_pos = putnbr(n / 10, color, pos);
        putnbr(n % 10, color, cur_pos)
    } else {
        putchar(b'0' + n as u8, color, pos);
        pos + 1
    }
}

pub fn printk(str: &[u8], color: Colors, args: &[PrintkArg]) {
    let mut arg_idx = 0;
    let mut i = 0;
    
    while i < str.len() {
        let c = str[i];
        
        if c == b'%' && i + 1 < str.len() {
            match str[i + 1] {
                b'd' => {
                    if let Some(PrintkArg::Int(n)) = args.get(arg_idx) {
                        printk_nbr(*n as isize, color);
                    }
                    arg_idx += 1;
                }
                b's' => {
                    if let Some(PrintkArg::Str(s)) = args.get(arg_idx) {
                        printk_str(s, color);
                    }
                    arg_idx += 1;
                }
                b'x' => {
                    if let Some(PrintkArg::Hex(n)) = args.get(arg_idx) {
                        printk_hex(*n, color);
                    }
                    arg_idx += 1;
                }
                b'%' => {
                    printk_char(b'%', color);
                }
                _ => {
                    printk_char(b'%', color);
                    printk_char(str[i + 1], color);
                }
            }
            i += 2;
        } else {
            printk_char(c, color);
            i += 1;
        }
    }
}

pub fn printk_char(c: u8, color: Colors) {
    unsafe {
        if CURSOR_ROW >= 25 {
            CURSOR_ROW = 24;
        }
        if c == b'\n' {
            CURSOR_COL = 0;
            CURSOR_ROW += 1;
        } else {
            putchar(c, color, CURSOR_ROW * 80 + CURSOR_COL);
            CURSOR_COL += 1;
            if CURSOR_COL >= 80 {
                CURSOR_COL = 0;
                CURSOR_ROW += 1;
            }
        }
    }
}

pub fn printk_str(s: &[u8], color: Colors) {
    for &c in s {
        if c == 0 {break;}
        printk_char(c, color);
    }
}

pub fn printk_nbr(n: isize, color: Colors) {
    if n < 0 {
        printk_char(b'-', color);
        printk_nbr(-n, color);
    } else if n > 9 {
        printk_nbr(n / 10, color);
        printk_nbr(n % 10, color);
    } else {
        printk_char(b'0' + n as u8, color);
    }
}

pub fn printk_hex(n: u32, color: Colors) {
    let digits = b"0123456789ABCDEF";
    printk_char(b'0', color);
    printk_char(b'x', color);
    for i in (0..8).rev() {
        let nibble = ((n >> (i * 4)) & 0xF) as usize;
        printk_char(digits[nibble], color);
    }
}

pub fn clear_screen() {
    for i in 0..(80 * 25) {
        putchar(b' ', Colors::Black, i);
    }
}
