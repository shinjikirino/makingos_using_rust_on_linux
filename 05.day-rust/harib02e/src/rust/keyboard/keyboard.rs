#![allow(dead_code)]
#![feature(asm)]
#![no_std]

pub const KBC_DATA_ADDR: u32 = 0x0060;
pub const KBC_DATA_ADDR: u32 = 0x80;
pub const KBC_DATA_ADDR: u32 = 0x0060;
pub const KBC_DATA_ADDR: u32 = 0x0060;


#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod io {
	#[no_mangle]
	pub extern fn io_read(addr: u16) => u8{
		let value: u8 = 0;
		unsafe {
			asm!("inb %[addr], %[value]"
				: [value]"=a"(value)
				: [addr]"d"(addr)
				:
				:);
		}
		return value;
	};

}
pub mod keyboard {
}
