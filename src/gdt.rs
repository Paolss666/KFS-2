const GDT_ADDR: u32 = 0x00000800;
const GDT_ENTRIES: usize = 7;

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct GdtEntry {
    limit_low:   u16,
    base_low:    u16,
    base_middle: u8,
    access:      u8,
    granularity: u8,
    base_high:   u8,
}

#[repr(C, packed)]
struct GdtPtr {
    limit: u16,
    base:  u32,
}

extern "C" {
    fn gdt_flush(ptr: *const GdtPtr);
}

fn make_entry(base: u32, limit: u32, access: u8, gran: u8) -> GdtEntry {
    GdtEntry {
        limit_low:   (limit & 0xFFFF) as u16,
        base_low:    (base  & 0xFFFF) as u16,
        base_middle: ((base >> 16) & 0xFF) as u8,
        access,
        granularity: (((limit >> 16) & 0x0F) as u8) | (gran & 0xF0),
        base_high:   ((base >> 24) & 0xFF) as u8,
    }
}

pub fn init() {
    let entries: [GdtEntry; GDT_ENTRIES] = [
        make_entry(0, 0,       0x00, 0x00), // 0x00 Null
        make_entry(0, 0xFFFFF, 0x9A, 0xCF), // 0x08 Kernel code
        make_entry(0, 0xFFFFF, 0x92, 0xCF), // 0x10 Kernel data
        make_entry(0, 0xFFFFF, 0x92, 0xCF), // 0x18 Kernel stack
        make_entry(0, 0xFFFFF, 0xFA, 0xCF), // 0x20 User code
        make_entry(0, 0xFFFFF, 0xF2, 0xCF), // 0x28 User data
        make_entry(0, 0xFFFFF, 0xF2, 0xCF), // 0x30 User stack
    ];

    let dst = GDT_ADDR as *mut GdtEntry;
    unsafe {
        for i in 0..GDT_ENTRIES {
            core::ptr::write(dst.add(i), entries[i]);
        }
    }

    let ptr = GdtPtr {
        limit: (core::mem::size_of::<GdtEntry>() * GDT_ENTRIES - 1) as u16,
        base:  GDT_ADDR,
    };

    unsafe { gdt_flush(&ptr as *const GdtPtr); }
}
