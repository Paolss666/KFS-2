pub const VGA_BUFFER: *mut u8 = 0xB8000 as *mut u8;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Colors{
	Black = 0x0,
	Blue = 0x1,
	Green = 0x2,
	Cyan = 0x3,
	Red = 0x4,
	Magenta = 0x5,
	Brown = 0x6,
	Gray = 0x7,
	DarkGray = 0x8,
	LightBlue = 0x9,
	LightGreen = 0xA,
	LightCyan = 0xB,
	LightRed = 0xC,
	LightMagenta = 0xD,
	Yellow = 0xE,
	White= 0xF,
}