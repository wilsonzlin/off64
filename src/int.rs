use crate::Off64;

// Short convenient macros for converting between int types without using the unsafe `as` operator.
#[macro_export]
macro_rules! isz {
  ($v:expr) => {
    isize::try_from($v).unwrap()
  };
}
#[macro_export]
macro_rules! i8 {
  ($v:expr) => {
    i8::try_from($v).unwrap()
  };
}
#[macro_export]
macro_rules! i16 {
  ($v:expr) => {
    i16::try_from($v).unwrap()
  };
}
#[macro_export]
macro_rules! i32 {
  ($v:expr) => {
    i32::try_from($v).unwrap()
  };
}
#[macro_export]
macro_rules! i64 {
  ($v:expr) => {
    i64::try_from($v).unwrap()
  };
}
#[macro_export]
macro_rules! usz {
  ($v:expr) => {
    usize::try_from($v).unwrap()
  };
}
#[macro_export]
macro_rules! u8 {
  ($v:expr) => {
    u8::try_from($v).unwrap()
  };
}
#[macro_export]
macro_rules! u16 {
  ($v:expr) => {
    u16::try_from($v).unwrap()
  };
}
#[macro_export]
macro_rules! u32 {
  ($v:expr) => {
    u32::try_from($v).unwrap()
  };
}
#[macro_export]
macro_rules! u64 {
  ($v:expr) => {
    u64::try_from($v).unwrap()
  };
}
pub trait Off64Int: Off64 {
  fn read_i16_be_at(&self, offset: u64) -> i16 {
    let mut buf = [0u8; 2];
    buf[0..2].copy_from_slice(self.read_at(offset, 2));
    i16::from_be_bytes(buf)
  }

  fn write_i16_be_at(&mut self, offset: u64, value: i16) {
    let buf = value.to_be_bytes();
    self.write_at(offset, &buf[0..2]);
  }

  fn read_i16_le_at(&self, offset: u64) -> i16 {
    let mut buf = [0u8; 2];
    buf[0..2].copy_from_slice(self.read_at(offset, 2));
    i16::from_le_bytes(buf)
  }

  fn write_i16_le_at(&mut self, offset: u64, value: i16) {
    let buf = value.to_le_bytes();
    self.write_at(offset, &buf[0..2]);
  }

  fn read_u16_be_at(&self, offset: u64) -> u16 {
    let mut buf = [0u8; 2];
    buf[0..2].copy_from_slice(self.read_at(offset, 2));
    u16::from_be_bytes(buf)
  }

  fn write_u16_be_at(&mut self, offset: u64, value: u16) {
    let buf = value.to_be_bytes();
    self.write_at(offset, &buf[0..2]);
  }

  fn read_u16_le_at(&self, offset: u64) -> u16 {
    let mut buf = [0u8; 2];
    buf[0..2].copy_from_slice(self.read_at(offset, 2));
    u16::from_le_bytes(buf)
  }

  fn write_u16_le_at(&mut self, offset: u64, value: u16) {
    let buf = value.to_le_bytes();
    self.write_at(offset, &buf[0..2]);
  }

  fn read_i24_be_at(&self, offset: u64) -> i32 {
    let mut buf = [0u8; 4];
    buf[1..4].copy_from_slice(self.read_at(offset, 3));
    i32::from_be_bytes(buf)
  }

  fn write_i24_be_at(&mut self, offset: u64, value: i32) {
    let buf = value.to_be_bytes();
    self.write_at(offset, &buf[1..4]);
  }

  fn read_i24_le_at(&self, offset: u64) -> i32 {
    let mut buf = [0u8; 4];
    buf[0..3].copy_from_slice(self.read_at(offset, 3));
    i32::from_le_bytes(buf)
  }

  fn write_i24_le_at(&mut self, offset: u64, value: i32) {
    let buf = value.to_le_bytes();
    self.write_at(offset, &buf[0..3]);
  }

  fn read_u24_be_at(&self, offset: u64) -> u32 {
    let mut buf = [0u8; 4];
    buf[1..4].copy_from_slice(self.read_at(offset, 3));
    u32::from_be_bytes(buf)
  }

  fn write_u24_be_at(&mut self, offset: u64, value: u32) {
    let buf = value.to_be_bytes();
    self.write_at(offset, &buf[1..4]);
  }

  fn read_u24_le_at(&self, offset: u64) -> u32 {
    let mut buf = [0u8; 4];
    buf[0..3].copy_from_slice(self.read_at(offset, 3));
    u32::from_le_bytes(buf)
  }

  fn write_u24_le_at(&mut self, offset: u64, value: u32) {
    let buf = value.to_le_bytes();
    self.write_at(offset, &buf[0..3]);
  }

  fn read_i32_be_at(&self, offset: u64) -> i32 {
    let mut buf = [0u8; 4];
    buf[0..4].copy_from_slice(self.read_at(offset, 4));
    i32::from_be_bytes(buf)
  }

  fn write_i32_be_at(&mut self, offset: u64, value: i32) {
    let buf = value.to_be_bytes();
    self.write_at(offset, &buf[0..4]);
  }

  fn read_i32_le_at(&self, offset: u64) -> i32 {
    let mut buf = [0u8; 4];
    buf[0..4].copy_from_slice(self.read_at(offset, 4));
    i32::from_le_bytes(buf)
  }

  fn write_i32_le_at(&mut self, offset: u64, value: i32) {
    let buf = value.to_le_bytes();
    self.write_at(offset, &buf[0..4]);
  }

  fn read_u32_be_at(&self, offset: u64) -> u32 {
    let mut buf = [0u8; 4];
    buf[0..4].copy_from_slice(self.read_at(offset, 4));
    u32::from_be_bytes(buf)
  }

  fn write_u32_be_at(&mut self, offset: u64, value: u32) {
    let buf = value.to_be_bytes();
    self.write_at(offset, &buf[0..4]);
  }

  fn read_u32_le_at(&self, offset: u64) -> u32 {
    let mut buf = [0u8; 4];
    buf[0..4].copy_from_slice(self.read_at(offset, 4));
    u32::from_le_bytes(buf)
  }

  fn write_u32_le_at(&mut self, offset: u64, value: u32) {
    let buf = value.to_le_bytes();
    self.write_at(offset, &buf[0..4]);
  }

  fn read_i40_be_at(&self, offset: u64) -> i64 {
    let mut buf = [0u8; 8];
    buf[3..8].copy_from_slice(self.read_at(offset, 5));
    i64::from_be_bytes(buf)
  }

  fn write_i40_be_at(&mut self, offset: u64, value: i64) {
    let buf = value.to_be_bytes();
    self.write_at(offset, &buf[3..8]);
  }

  fn read_i40_le_at(&self, offset: u64) -> i64 {
    let mut buf = [0u8; 8];
    buf[0..5].copy_from_slice(self.read_at(offset, 5));
    i64::from_le_bytes(buf)
  }

  fn write_i40_le_at(&mut self, offset: u64, value: i64) {
    let buf = value.to_le_bytes();
    self.write_at(offset, &buf[0..5]);
  }

  fn read_u40_be_at(&self, offset: u64) -> u64 {
    let mut buf = [0u8; 8];
    buf[3..8].copy_from_slice(self.read_at(offset, 5));
    u64::from_be_bytes(buf)
  }

  fn write_u40_be_at(&mut self, offset: u64, value: u64) {
    let buf = value.to_be_bytes();
    self.write_at(offset, &buf[3..8]);
  }

  fn read_u40_le_at(&self, offset: u64) -> u64 {
    let mut buf = [0u8; 8];
    buf[0..5].copy_from_slice(self.read_at(offset, 5));
    u64::from_le_bytes(buf)
  }

  fn write_u40_le_at(&mut self, offset: u64, value: u64) {
    let buf = value.to_le_bytes();
    self.write_at(offset, &buf[0..5]);
  }

  fn read_i48_be_at(&self, offset: u64) -> i64 {
    let mut buf = [0u8; 8];
    buf[2..8].copy_from_slice(self.read_at(offset, 6));
    i64::from_be_bytes(buf)
  }

  fn write_i48_be_at(&mut self, offset: u64, value: i64) {
    let buf = value.to_be_bytes();
    self.write_at(offset, &buf[2..8]);
  }

  fn read_i48_le_at(&self, offset: u64) -> i64 {
    let mut buf = [0u8; 8];
    buf[0..6].copy_from_slice(self.read_at(offset, 6));
    i64::from_le_bytes(buf)
  }

  fn write_i48_le_at(&mut self, offset: u64, value: i64) {
    let buf = value.to_le_bytes();
    self.write_at(offset, &buf[0..6]);
  }

  fn read_u48_be_at(&self, offset: u64) -> u64 {
    let mut buf = [0u8; 8];
    buf[2..8].copy_from_slice(self.read_at(offset, 6));
    u64::from_be_bytes(buf)
  }

  fn write_u48_be_at(&mut self, offset: u64, value: u64) {
    let buf = value.to_be_bytes();
    self.write_at(offset, &buf[2..8]);
  }

  fn read_u48_le_at(&self, offset: u64) -> u64 {
    let mut buf = [0u8; 8];
    buf[0..6].copy_from_slice(self.read_at(offset, 6));
    u64::from_le_bytes(buf)
  }

  fn write_u48_le_at(&mut self, offset: u64, value: u64) {
    let buf = value.to_le_bytes();
    self.write_at(offset, &buf[0..6]);
  }

  fn read_i56_be_at(&self, offset: u64) -> i64 {
    let mut buf = [0u8; 8];
    buf[1..8].copy_from_slice(self.read_at(offset, 7));
    i64::from_be_bytes(buf)
  }

  fn write_i56_be_at(&mut self, offset: u64, value: i64) {
    let buf = value.to_be_bytes();
    self.write_at(offset, &buf[1..8]);
  }

  fn read_i56_le_at(&self, offset: u64) -> i64 {
    let mut buf = [0u8; 8];
    buf[0..7].copy_from_slice(self.read_at(offset, 7));
    i64::from_le_bytes(buf)
  }

  fn write_i56_le_at(&mut self, offset: u64, value: i64) {
    let buf = value.to_le_bytes();
    self.write_at(offset, &buf[0..7]);
  }

  fn read_u56_be_at(&self, offset: u64) -> u64 {
    let mut buf = [0u8; 8];
    buf[1..8].copy_from_slice(self.read_at(offset, 7));
    u64::from_be_bytes(buf)
  }

  fn write_u56_be_at(&mut self, offset: u64, value: u64) {
    let buf = value.to_be_bytes();
    self.write_at(offset, &buf[1..8]);
  }

  fn read_u56_le_at(&self, offset: u64) -> u64 {
    let mut buf = [0u8; 8];
    buf[0..7].copy_from_slice(self.read_at(offset, 7));
    u64::from_le_bytes(buf)
  }

  fn write_u56_le_at(&mut self, offset: u64, value: u64) {
    let buf = value.to_le_bytes();
    self.write_at(offset, &buf[0..7]);
  }

  fn read_i64_be_at(&self, offset: u64) -> i64 {
    let mut buf = [0u8; 8];
    buf[0..8].copy_from_slice(self.read_at(offset, 8));
    i64::from_be_bytes(buf)
  }

  fn write_i64_be_at(&mut self, offset: u64, value: i64) {
    let buf = value.to_be_bytes();
    self.write_at(offset, &buf[0..8]);
  }

  fn read_i64_le_at(&self, offset: u64) -> i64 {
    let mut buf = [0u8; 8];
    buf[0..8].copy_from_slice(self.read_at(offset, 8));
    i64::from_le_bytes(buf)
  }

  fn write_i64_le_at(&mut self, offset: u64, value: i64) {
    let buf = value.to_le_bytes();
    self.write_at(offset, &buf[0..8]);
  }

  fn read_u64_be_at(&self, offset: u64) -> u64 {
    let mut buf = [0u8; 8];
    buf[0..8].copy_from_slice(self.read_at(offset, 8));
    u64::from_be_bytes(buf)
  }

  fn write_u64_be_at(&mut self, offset: u64, value: u64) {
    let buf = value.to_be_bytes();
    self.write_at(offset, &buf[0..8]);
  }

  fn read_u64_le_at(&self, offset: u64) -> u64 {
    let mut buf = [0u8; 8];
    buf[0..8].copy_from_slice(self.read_at(offset, 8));
    u64::from_le_bytes(buf)
  }

  fn write_u64_le_at(&mut self, offset: u64, value: u64) {
    let buf = value.to_le_bytes();
    self.write_at(offset, &buf[0..8]);
  }
}

pub fn create_i16_be(value: i16) -> [u8; 2] {
  let buf = value.to_be_bytes();
  buf[0..2].try_into().unwrap()
}

pub fn create_i16_le(value: i16) -> [u8; 2] {
  let buf = value.to_le_bytes();
  buf[0..2].try_into().unwrap()
}

pub fn create_u16_be(value: u16) -> [u8; 2] {
  let buf = value.to_be_bytes();
  buf[0..2].try_into().unwrap()
}

pub fn create_u16_le(value: u16) -> [u8; 2] {
  let buf = value.to_le_bytes();
  buf[0..2].try_into().unwrap()
}

pub fn create_i24_be(value: i32) -> [u8; 3] {
  let buf = value.to_be_bytes();
  buf[1..4].try_into().unwrap()
}

pub fn create_i24_le(value: i32) -> [u8; 3] {
  let buf = value.to_le_bytes();
  buf[0..3].try_into().unwrap()
}

pub fn create_u24_be(value: u32) -> [u8; 3] {
  let buf = value.to_be_bytes();
  buf[1..4].try_into().unwrap()
}

pub fn create_u24_le(value: u32) -> [u8; 3] {
  let buf = value.to_le_bytes();
  buf[0..3].try_into().unwrap()
}

pub fn create_i32_be(value: i32) -> [u8; 4] {
  let buf = value.to_be_bytes();
  buf[0..4].try_into().unwrap()
}

pub fn create_i32_le(value: i32) -> [u8; 4] {
  let buf = value.to_le_bytes();
  buf[0..4].try_into().unwrap()
}

pub fn create_u32_be(value: u32) -> [u8; 4] {
  let buf = value.to_be_bytes();
  buf[0..4].try_into().unwrap()
}

pub fn create_u32_le(value: u32) -> [u8; 4] {
  let buf = value.to_le_bytes();
  buf[0..4].try_into().unwrap()
}

pub fn create_i40_be(value: i64) -> [u8; 5] {
  let buf = value.to_be_bytes();
  buf[3..8].try_into().unwrap()
}

pub fn create_i40_le(value: i64) -> [u8; 5] {
  let buf = value.to_le_bytes();
  buf[0..5].try_into().unwrap()
}

pub fn create_u40_be(value: u64) -> [u8; 5] {
  let buf = value.to_be_bytes();
  buf[3..8].try_into().unwrap()
}

pub fn create_u40_le(value: u64) -> [u8; 5] {
  let buf = value.to_le_bytes();
  buf[0..5].try_into().unwrap()
}

pub fn create_i48_be(value: i64) -> [u8; 6] {
  let buf = value.to_be_bytes();
  buf[2..8].try_into().unwrap()
}

pub fn create_i48_le(value: i64) -> [u8; 6] {
  let buf = value.to_le_bytes();
  buf[0..6].try_into().unwrap()
}

pub fn create_u48_be(value: u64) -> [u8; 6] {
  let buf = value.to_be_bytes();
  buf[2..8].try_into().unwrap()
}

pub fn create_u48_le(value: u64) -> [u8; 6] {
  let buf = value.to_le_bytes();
  buf[0..6].try_into().unwrap()
}

pub fn create_i56_be(value: i64) -> [u8; 7] {
  let buf = value.to_be_bytes();
  buf[1..8].try_into().unwrap()
}

pub fn create_i56_le(value: i64) -> [u8; 7] {
  let buf = value.to_le_bytes();
  buf[0..7].try_into().unwrap()
}

pub fn create_u56_be(value: u64) -> [u8; 7] {
  let buf = value.to_be_bytes();
  buf[1..8].try_into().unwrap()
}

pub fn create_u56_le(value: u64) -> [u8; 7] {
  let buf = value.to_le_bytes();
  buf[0..7].try_into().unwrap()
}

pub fn create_i64_be(value: i64) -> [u8; 8] {
  let buf = value.to_be_bytes();
  buf[0..8].try_into().unwrap()
}

pub fn create_i64_le(value: i64) -> [u8; 8] {
  let buf = value.to_le_bytes();
  buf[0..8].try_into().unwrap()
}

pub fn create_u64_be(value: u64) -> [u8; 8] {
  let buf = value.to_be_bytes();
  buf[0..8].try_into().unwrap()
}

pub fn create_u64_le(value: u64) -> [u8; 8] {
  let buf = value.to_le_bytes();
  buf[0..8].try_into().unwrap()
}
