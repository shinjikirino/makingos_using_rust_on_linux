#![feature(lang_items)]
#![feature(start)]
#![no_main]
#![feature(no_std)]
#![no_std]

#[no_mangle]
extern {
	fn io_hlt();
	fn io_load_eflags() -> i32;
	fn io_cli();
	fn io_out8(addr: u32, color: u32);
	fn io_store_eflags(addr: i32);
}

#[no_mangle]
pub extern fn init_palate(start: u32, end: u32) {
	let COLOR_TABLE : [u8; 48] = [
		0x00u8, 0x00u8, 0x00u8,   // 0: black
		0xffu8, 0x00u8, 0x00u8,   // 1: red
		0x00u8, 0xffu8, 0x00u8,   // 2: lime
		0xffu8, 0xffu8, 0x00u8,   // 3: yellow
		0x00u8, 0x00u8, 0xffu8,   // 4: blue
		0xffu8, 0x00u8, 0xffu8,   // 5: purple
		0x00u8, 0xffu8, 0xffu8,   // 6: cyan
		0xffu8, 0xffu8, 0xffu8,   // 7: white
		0xc6u8, 0xc6u8, 0xc6u8,   // 8: gray
		0x84u8, 0x00u8, 0x00u8,   // 9: dark red
		0x00u8, 0x84u8, 0x00u8,   // 10: green
		0x84u8, 0x84u8, 0x00u8,   // 11: dark yellow
		0x00u8, 0x00u8, 0x84u8,   // 12: dark blue
		0x84u8, 0x00u8, 0x84u8,   // 13: dark purple
		0x00u8, 0x84u8, 0x84u8,   // 14: dark cyan
		0x84u8, 0x84u8, 0x84u8    // 15: dark gray
	];

	unsafe {
		let eflags: i32 = io_load_eflags();
		io_cli();
		let mut count = 0;
		io_out8(0x03c8, start);
		for x in start..end  {
			io_out8(0x03c9, (COLOR_TABLE[count] / 4) as u32);
			io_out8(0x03c9, (COLOR_TABLE[count + 1] / 4) as u32);
			io_out8(0x03c9, (COLOR_TABLE[count + 2] / 4) as u32);
			count += 3;
		}
		io_store_eflags(eflags);
	}
}

#[no_mangle]
pub extern fn hlt() {
	unsafe {
		io_hlt();
	}
}

#[no_mangle]
#[start]
pub extern fn Main() {
	init_palate(0, 15);
	for x in 0x000a0000..0x000affff {
		let raw = x as *mut isize;
		unsafe {
			*raw = x & 0x0f;
		}
	}
	loop {
		hlt()
	}
}

#[no_mangle]
#[lang = "eh_personality"]
extern fn eh_personality() {}

#[no_mangle]
#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! { loop {} }
