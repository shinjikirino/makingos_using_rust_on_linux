#![feature(lang_items)]
#![feature(start)]
#![no_main]
#![feature(no_std)]
#![no_std]


macro_rules! COL8_000000 {() => (0);}
macro_rules! COL8_FF0000 {() => (1);}
macro_rules! COL8_00FF00 {() => (2);}
macro_rules! COL8_FFFF00 {() => (3);}
macro_rules! COL8_0000FF {() => (4);}
macro_rules! COL8_FF00FF {() => (5);}
macro_rules! COL8_00FFFF {() => (6);}
macro_rules! COL8_FFFFFF {() => (7);}
macro_rules! COL8_C6C6C6 {() => (8);}
macro_rules! COL8_840000 {() => (9);}
macro_rules! COL8_008400 {() => (10);}
macro_rules! COL8_848400 {() => (11);}
macro_rules! COL8_000084 {() => (12);}
macro_rules! COL8_840084 {() => (13);}
macro_rules! COL8_008484 {() => (14);}
macro_rules! COL8_848484 {() => (15);}

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
pub extern fn boxfill8(vram: &i32, xsize: i32, c: u8, x0: i32, y0: i32, x1: i32, y1: i32) {
	for y in y0..y1 {
		for x in x0..x1 {
			unsafe {
				let p_vram = (vram + (y * xsize + x)) as *mut u8;
				*p_vram = c;
			}
		}
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
	let p = 0xa0000 as i32;
	boxfill8(&p, 320, COL8_FF0000!(), 20, 20, 120, 120);
	boxfill8(&p, 320, COL8_00FF00!(), 70, 50, 127, 150);
	boxfill8(&p, 320, COL8_0000FF!(), 120, 80, 220, 180);
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
