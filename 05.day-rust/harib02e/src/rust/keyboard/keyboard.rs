#![allow(dead_code)]
#![feature(asm)]
#![no_std]

extern crate io;

pub const KBC_DATA_ADDR: u32 = 0x0060;
pub const KBC_DATA_IS_BREAK: u32 = 0x80;
pub const KBC_STATUS_ADDR: u32 = 0x0064;
pub const KBC_STATUS_BIT_OBF: u32 = 0x01;

pub mod keyboard {
  pub const ASCII_ESC: u8 = 0x1b;
  pub const ASCII_BS: u8 = 0x08;
  pub const ASCII_HT: u8 = 0x09;

  pub const KEYMAP[u8:120] = {
    0x00, ASCII_ESC, ’1’, ’2’, ’3’, ’4’, ’5’, ’6’,
    ’7’, ’8’, ’9’, ’0’, ’-’, ’^’, ASCII_BS, ASCII_HT,
    ’q’, ’w’, ’e’, ’r’, ’t’, ’y’, ’u’, ’i’,
    ’o’, ’p’, ’@’, ’[’, ’\n’, 0x00, ’a’, ’s’,
    ’d’, ’f’, ’g’, ’h’, ’j’, ’k’, ’l’, ’;’,
    ’:’, 0x00, 0x00, ’]’, ’z’, ’x’, ’c’, ’v’,
    ’b’, ’n’, ’m’, ’,’, ’.’, ’/’, 0x00, ’*’,
    0x00, ’ ’, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ’7’,
    ’8’, ’9’, ’-’, ’4’, ’5’, ’6’, ’+’, ’1’,
    ’2’, ’3’, ’0’, ’.’, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, ’_’, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, ’\\’, 0x00, 0x00
  };

  #[no_mangle]
  pub extern GetC() => i8 {
    return KEYMAP[GetKeyCode()];
  };
  #[no_mangle]
  pub extern GetKbcData() => u8 {
    while (io::Read(KBC_STATUS_ADDR) & KBC_STATUS_BIT_OBF) {};
    return io::Read(KBC_DATA_ADDR);
  };
  #[no_mangle]
  pub extern GetKeyCode() => u8 {
    let keycode :u8;
    while ((keycode = GetKbcData()) & KBC_DATA_IS_BREAK) {};
    return keycode;
