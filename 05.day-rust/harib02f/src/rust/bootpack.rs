#![feature(lang_items)]
#![feature(start)]
#![no_main]
#![feature(no_std)]
#![feature(panic_implementation)]
#![no_std]

use core::panic::PanicInfo;
extern crate font;
extern crate keyboard;
extern crate io;

pub const COL8_000000: u8 = 0;
pub const COL8_FF0000: u8 = 1;
pub const COL8_00FF00: u8 = 2;
pub const COL8_FFFF00: u8 = 3;
pub const COL8_0000FF: u8 = 4;
pub const COL8_FF00FF: u8 = 5;
pub const COL8_00FFFF: u8 = 6;
pub const COL8_FFFFFF: u8 = 7;
pub const COL8_C6C6C6: u8 = 8;
pub const COL8_840000: u8 = 9;
pub const COL8_008400: u8 = 10;
pub const COL8_848400: u8 = 11;
pub const COL8_000084: u8 = 12;
pub const COL8_840084: u8 = 13;
pub const COL8_008484: u8 = 14;
pub const COL8_848484: u8 = 15;

pub const BOOTINFO_ADDR: u32 = 0x0ff0;

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
pub extern fn boxfill8(vram: i32, xsize: i32, c: u8, x0: i32, y0: i32, x1: i32, y1: i32) {
	for y in y0..y1 + 1 {  // rustのforは値自身は含まない
		for x in x0..x1 + 1 {  // rustのforは値自身は含まない
			unsafe {
				let p_vram = (vram + (y * xsize + x)) as *mut u8;
				*p_vram = c;
			}
		}
	}
}

#[no_mangle]
pub extern fn init_screen(vram:i32, xsize:i32, ysize:i32) {
	boxfill8(vram, xsize, COL8_008484, 0, 			0, 			xsize - 1, 	ysize - 29	);
	boxfill8(vram, xsize, COL8_C6C6C6, 0, 			ysize - 28, xsize - 1, 	ysize - 28	);
	boxfill8(vram, xsize, COL8_FFFFFF, 0, 			ysize - 27, xsize - 1, 	ysize - 27	);
	boxfill8(vram, xsize, COL8_C6C6C6, 0, 			ysize - 26, xsize - 1, 	ysize - 1	);

	boxfill8(vram, xsize, COL8_FFFFFF, 3, 			ysize - 24, 59, 		ysize - 24	);
	boxfill8(vram, xsize, COL8_FFFFFF, 2, 			ysize - 24, 2, 			ysize - 4	);
	boxfill8(vram, xsize, COL8_848484, 3, 			ysize - 4, 	59, 		ysize - 4	);
	boxfill8(vram, xsize, COL8_848484, 59, 			ysize - 23, 59, 		ysize - 5	);
	boxfill8(vram, xsize, COL8_000000, 2, 			ysize - 3, 	59, 		ysize - 3	);
	boxfill8(vram, xsize, COL8_000000, 60, 			ysize - 24, 60, 		ysize - 3	);

	boxfill8(vram, xsize, COL8_848484, xsize - 47, 	ysize - 24, xsize - 4, 	ysize - 24	);
	boxfill8(vram, xsize, COL8_848484, xsize - 47, 	ysize - 23, xsize - 47, ysize - 4	);
	boxfill8(vram, xsize, COL8_FFFFFF, xsize - 47, 	ysize - 3, 	xsize - 4, 	ysize - 3	);
	boxfill8(vram, xsize, COL8_FFFFFF, xsize - 3, 	ysize - 24, xsize - 3, 	ysize - 3	);
}


#[no_mangle]
pub extern fn hlt() {
	unsafe {
		io_hlt();
	}
}

#[no_mangle]
// アライメントに注意。参考：https://ryochack.hatenablog.com/entry/2018/03/23/184943
#[repr(packed)]  // pragma pack と同等
struct Bootinfo {
	cyls: u8,
	leds: u8,
	vmode: u8,
	reserve: u8,
	scrnx: i16,
	scrny: i16,
	vram: i32,
}


#[no_mangle]
pub extern fn putfont8(vram: i32, xsize: i32, x: i32, y: i32, c:u8, c_array:*mut u8) {
	// for i in 0..16 {  // forだとうまく行かない。なぜかloopだとうまく行く。ダサい
	let mut i = 0;
	loop {
		let addr = (vram + (y + i) * xsize + x) as i32;
		// let d = c_array[i as usize] as u8;
		unsafe {
			let d = *((c_array as u32 + i as u32) as *mut u8);
			if ((d & 0x80) != 0) {
				let p = addr as *mut u8;
				*p = c;
			}
			if ((d & 0x40) != 0) {
				let p = (addr + 1) as *mut u8;
				*p = c;
			}
			if ((d & 0x20) != 0) {
				let p = (addr + 2) as *mut u8;
				*p = c;
			}
			if ((d & 0x10) != 0) {
				let p = (addr + 3) as *mut u8;
				*p = c;
			}
			if ((d & 0x08) != 0) {
				let p = (addr + 4) as *mut u8;
				*p = c;
			}
			if ((d & 0x04) != 0) {
				let p = (addr + 5) as *mut u8;
				*p = c;
			}
			if ((d & 0x02) != 0) {
				let p = (addr + 6) as *mut u8;
				*p = c;
			}
			if ((d & 0x01) != 0) {
				let p = (addr + 7) as *mut u8;
				*p = c;
			}
		}
		i = i + 1;
		if (i == (16 - 1) as i32) {break;}  // ダサいからいずれ原因救命してforに戻す
	}
	return;
}

#[no_mangle]
// pub extern fn putfonts8_asc(vram: i32, xsize: i32, x: i32, y: i32, c:u8, c_array:*mut u8) {
pub extern fn putfonts8_asc(vram: i32, xsize: i32, x: i32, y: i32, color:u8, c_array: &str) {
	// loop {
	for c in c_array.chars() {
		let ascii : usize;
		unsafe {
			putfont8(vram, xsize , x, y, color, &mut font::font::HANKAKU[(c as usize * 16)] as *mut u8);
		}
	}

}


#[no_mangle]
#[start]
pub extern fn Main() {
	let p_bootinfo = BOOTINFO_ADDR as *mut Bootinfo;
	
	init_palate(0, 15);

	unsafe {  // 構造体のポインタの先を見に行くのでunsafe。いずれunsafeブロックを使用しないようにする

		init_screen((*p_bootinfo).vram, (*p_bootinfo).scrnx as i32, (*p_bootinfo).scrny as i32);
		let indexA = 'A' as usize * 16;
		let indexB = 'B' as usize * 16;
		let indexC = 'C' as usize * 16;
		let index1 = '1' as usize * 16;
		let index2 = '2' as usize * 16;
		let index3 = '3' as usize * 16;
		putfont8((*p_bootinfo).vram, (*p_bootinfo).scrnx as i32, 8, 10, COL8_FFFFFF, &mut font::font::HANKAKU[indexA] as *mut u8);
		putfont8((*p_bootinfo).vram, (*p_bootinfo).scrnx as i32, 16, 10, COL8_FFFFFF, &mut font::font::HANKAKU[indexB] as *mut u8);
		putfont8((*p_bootinfo).vram, (*p_bootinfo).scrnx as i32, 24, 10, COL8_FFFFFF, &mut font::font::HANKAKU[indexC] as *mut u8);
		putfont8((*p_bootinfo).vram, (*p_bootinfo).scrnx as i32, 40, 10, COL8_FFFFFF, &mut font::font::HANKAKU[index1] as *mut u8);
		putfont8((*p_bootinfo).vram, (*p_bootinfo).scrnx as i32, 48, 10, COL8_FFFFFF, &mut font::font::HANKAKU[index2] as *mut u8);
		putfont8((*p_bootinfo).vram, (*p_bootinfo).scrnx as i32, 56, 10, COL8_FFFFFF, &mut font::font::HANKAKU[index3] as *mut u8);
		
		let indexKey = keyboard::keyboard::GetC() as usize * 16;
		putfonts8_asc((*p_bootinfo).vram, (*p_bootinfo).scrnx as i32, 8, 20, COL8_FFFFFF, "Test");

		putfont8((*p_bootinfo).vram, (*p_bootinfo).scrnx as i32, 64, 20, COL8_FFFFFF, &mut font::font::HANKAKU[indexKey] as *mut u8);
	}

	loop {
		hlt()
	}
}


#[no_mangle]
#[lang = "eh_personality"]
extern fn eh_personality() {}

#[no_mangle]
#[lang = "panic_impl"]
#[panic_implementation]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
