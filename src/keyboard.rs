use core::cell::Cell;
use crate::io::inb;

const KB_DATA:   u16 = 0x60;
const KB_STATUS: u16 = 0x64;

// ── Trait (Dependency-Inversion: Shell depends on this, not on Ps2Keyboard) ──

pub trait KeyboardDriver {
    /// Return the next printable ASCII byte if a key was just pressed,
    /// `None` if nothing is available yet.
    fn poll(&self) -> Option<u8>;
}

// ── Scancode Set 1 — US QWERTY ────────────────────────────────────────────

static NORMAL_MAP: [u8; 58] = [
    0,       // 0x00 — undefined
    0x1B,    // 0x01 — ESC
    b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', // 0x02-0x0B
    b'-', b'=',  // 0x0C-0x0D
    0x08,    // 0x0E — Backspace
    b'\t',   // 0x0F — Tab
    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', // 0x10-0x19
    b'[', b']',  // 0x1A-0x1B
    b'\n',   // 0x1C — Enter
    0,       // 0x1D — Left Ctrl
    b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', // 0x1E-0x26
    b';', b'\'', b'`', // 0x27-0x29
    0,       // 0x2A — Left Shift
    b'\\',   // 0x2B
    b'z', b'x', b'c', b'v', b'b', b'n', b'm', // 0x2C-0x32
    b',', b'.', b'/', // 0x33-0x35
    0,       // 0x36 — Right Shift
    b'*',    // 0x37 — Keypad *
    0,       // 0x38 — Left Alt (handled explicitly in poll, never reaches table)
    b' ',    // 0x39 — Space
];

static SHIFT_MAP: [u8; 58] = [
    0,
    0x1B,
    b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')',
    b'_', b'+',
    0x08,
    b'\t',
    b'Q', b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P',
    b'{', b'}',
    b'\n',
    0,
    b'A', b'S', b'D', b'F', b'G', b'H', b'J', b'K', b'L',
    b':', b'"', b'~',
    0,
    b'|',
    b'Z', b'X', b'C', b'V', b'B', b'N', b'M',
    b'<', b'>', b'?',
    0,
    b'*',
    0,
    b' ',
];

// ── PS/2 keyboard driver (polling) ────────────────────────────────────────

pub struct Ps2Keyboard {
    shift: Cell<bool>,
}

impl Ps2Keyboard {
    pub const fn new() -> Self {
        Ps2Keyboard { shift: Cell::new(false) }
    }

    /// Flush the keyboard buffer (discard stale scancodes from BIOS).
    pub fn flush(&self) {
        unsafe {
            while inb(KB_STATUS) & 0x01 != 0 {
                let _ = inb(KB_DATA);
            }
        }
    }
}

impl KeyboardDriver for Ps2Keyboard {
    fn poll(&self) -> Option<u8> {
        unsafe {
            // Output-buffer-full bit
            if inb(KB_STATUS) & 0x01 == 0 {
                return None;
            }
            let sc = inb(KB_DATA);

            // Shift press / release
            if sc == 0x2A || sc == 0x36 { self.shift.set(true);  return None; }
            if sc == 0xAA || sc == 0xB6 { self.shift.set(false); return None; }
            // Alt press / release — explicitly ignored
            if sc == 0x38 || sc == 0xB8 { return None; }
            // Any other release (bit 7 set)
            if sc & 0x80 != 0 { return None; }

            let idx = sc as usize;
            if idx >= NORMAL_MAP.len() { return None; }

            let c = if self.shift.get() { SHIFT_MAP[idx] } else { NORMAL_MAP[idx] };
            if c != 0 { Some(c) } else { None }
        }
    }
}
