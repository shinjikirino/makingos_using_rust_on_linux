#![allow(dead_code)]
#![feature(asm)]
#![no_std]


pub mod keyboard {

  extern crate io;

  const KBC_DATA_ADDR: u16 = 0x0060;
  const KBC_DATA_IS_BREAK: u16 = 0x80;
  const KBC_STATUS_ADDR: u16 = 0x0064;
  const KBC_STATUS_BIT_OBF: u16 = 0x01;


  pub const ASCII_ESC: u8 = 0x1b;
  pub const ASCII_BS: u8 = 0x08;
  pub const ASCII_HT: u8 = 0x09;

  const KEYMAP:[u8;128] = [
    0x00, ASCII_ESC, '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8,
    '7' as u8, '8' as u8, '9' as u8, '0' as u8, '-' as u8, '^' as u8, ASCII_BS, ASCII_HT,
    'q' as u8, 'w' as u8, 'e' as u8, 'r' as u8, 't' as u8, 'y' as u8, 'u' as u8, 'i' as u8,
    'o' as u8, 'p' as u8, '@' as u8, '[' as u8, '\n' as u8, 0x00, 'a' as u8, 's' as u8,
    'd' as u8, 'f' as u8, 'g' as u8, 'h' as u8, 'j' as u8, 'k' as u8, 'l' as u8, ';' as u8,
    ':' as u8, 0x00, 0x00, ']' as u8, 'z' as u8, 'x' as u8, 'c' as u8, 'v' as u8,
    'b' as u8, 'n' as u8, 'm' as u8, ',' as u8, '.' as u8, '/' as u8, 0x00, '*' as u8,
    0x00, ' ' as u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, '7' as u8,
    '8' as u8, '9' as u8, '-' as u8, '4' as u8, '5' as u8, '6' as u8, '+' as u8, '1' as u8,
    '2' as u8, '3' as u8, '0' as u8, '.' as u8, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, '_' as u8, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, '\\' as u8, 0x00, 0x00
  ];

  #[no_mangle]
  pub extern fn GetC() -> u8 {
    return KEYMAP[GetKeyCode() as usize];
  }
  #[no_mangle]
  pub extern fn GetKbcData() -> u8 {
    while ((io::io::Read(KBC_STATUS_ADDR) as u16 & KBC_STATUS_BIT_OBF) != KBC_STATUS_BIT_OBF) {};
    return io::io::Read(KBC_DATA_ADDR);
  }
  #[no_mangle]
  pub extern fn GetKeyCode() -> u8 {
    let mut keycode :u8;
    loop {
      keycode = GetKbcData();
      if ((keycode as u16 & KBC_DATA_IS_BREAK) != KBC_DATA_IS_BREAK) {
        break;
      }
    }
    return keycode;
  }
}
