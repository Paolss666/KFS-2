use crate::types::VGA_BUFFER;
use crate::types::Colors;

const VGA_WIDTH:  usize = 80;
const VGA_HEIGHT: usize = 25;

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
        len += 1;
    }
    len
}

pub fn strcmp(s1: &[u8], s2: &[u8]) -> i32 {
    let mut i = 0;
    loop {
        let c1 = s1[i];
        let c2 = s2[i];
        if c1 != c2 || c1 == 0 {
            return (c1 as i32) - (c2 as i32);
        }
        i += 1;
    }
}

pub fn putchar(c: u8, color: Colors, pos: usize) {
    unsafe {
        core::ptr::write_volatile(VGA_BUFFER.add(pos * 2),     c);
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
                b'%' => { printk_char(b'%', color); }
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

// ── Scroll ────────────────────────────────────────────────────────────────

/// Scroll the VGA text buffer up by one line and clear the last row.
fn scroll() {
    unsafe {
        for row in 0..(VGA_HEIGHT - 1) {
            for col in 0..VGA_WIDTH {
                let src = (row + 1) * VGA_WIDTH + col;
                let dst =  row      * VGA_WIDTH + col;
                let ch  = core::ptr::read_volatile(VGA_BUFFER.add(src * 2));
                let att = core::ptr::read_volatile(VGA_BUFFER.add(src * 2 + 1));
                core::ptr::write_volatile(VGA_BUFFER.add(dst * 2),     ch);
                core::ptr::write_volatile(VGA_BUFFER.add(dst * 2 + 1), att);
            }
        }
        // Clear last row
        for col in 0..VGA_WIDTH {
            let pos = (VGA_HEIGHT - 1) * VGA_WIDTH + col;
            core::ptr::write_volatile(VGA_BUFFER.add(pos * 2),     b' ');
            core::ptr::write_volatile(VGA_BUFFER.add(pos * 2 + 1), Colors::Black as u8);
        }
        CURSOR_ROW = VGA_HEIGHT - 1;
        CURSOR_COL = 0;
    }
}

// ── Hardware cursor (VGA CRT registers) ──────────────────────────────────

fn update_hw_cursor() {
    unsafe {
        let pos = (CURSOR_ROW * VGA_WIDTH + CURSOR_COL) as u16;
        crate::io::outb(0x3D4, 0x0F);
        crate::io::outb(0x3D5, (pos & 0xFF) as u8);
        crate::io::outb(0x3D4, 0x0E);
        crate::io::outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
    }
}

// ── Public cursor helpers ─────────────────────────────────────────────────

pub fn reset_cursor() {
    unsafe {
        CURSOR_COL = 0;
        CURSOR_ROW = 0;
    }
    update_hw_cursor();
}

/// Erase the character immediately before the cursor.
pub fn backspace() {
    unsafe {
        if CURSOR_COL > 0 {
            CURSOR_COL -= 1;
        } else if CURSOR_ROW > 0 {
            CURSOR_ROW -= 1;
            CURSOR_COL = VGA_WIDTH - 1;
        } else {
            return;
        }
        putchar(b' ', Colors::Black, CURSOR_ROW * VGA_WIDTH + CURSOR_COL);
    }
    update_hw_cursor();
}

// ── Core output primitives ────────────────────────────────────────────────

pub fn printk_char(c: u8, color: Colors) {
    unsafe {
        if c == b'\n' {
            CURSOR_COL = 0;
            CURSOR_ROW += 1;
        } else {
            putchar(c, color, CURSOR_ROW * VGA_WIDTH + CURSOR_COL);
            CURSOR_COL += 1;
            if CURSOR_COL >= VGA_WIDTH {
                CURSOR_COL = 0;
                CURSOR_ROW += 1;
            }
        }
        if CURSOR_ROW >= VGA_HEIGHT {
            scroll();
        }
    }
    update_hw_cursor();
}

pub fn printk_str(s: &[u8], color: Colors) {
    for &c in s {
        if c == 0 { break; }
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
    for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
        putchar(b' ', Colors::Black, i);
    }
    reset_cursor();
}
