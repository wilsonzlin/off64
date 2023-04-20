use crate::chrono::Off64Chrono;
use crate::int::Off64Int;
use crate::usz;
use crate::Off64;

impl Off64 for [u8] {
  fn read_at(&self, offset: u64, len: u64) -> &[u8] {
    &self[usz!(offset)..usz!(offset + len)]
  }

  fn write_at<'v>(&mut self, offset: u64, value: &'v [u8]) {
    self[usz!(offset)..usz!(offset) + value.len()].copy_from_slice(value);
  }
}

impl Off64Chrono for [u8] {}
impl Off64Int for [u8] {}
