use crate::int::Off64ReadInt;
use crate::int::Off64WriteMutInt;
use crate::usz;
use crate::Off64Read;
use crate::Off64WriteMut;

impl<'a> Off64Read<'a, &'a [u8]> for [u8] {
  fn read_at(&'a self, offset: u64, len: u64) -> &'a [u8] {
    &self[usz!(offset)..usz!(offset + len)]
  }
}

impl Off64WriteMut<&[u8]> for [u8] {
  fn write_at<'v>(&mut self, offset: u64, value: &'v [u8]) {
    self[usz!(offset)..usz!(offset) + value.len()].copy_from_slice(value);
  }
}

impl<'a> Off64ReadInt<'a, &'a [u8]> for [u8] {}
impl Off64WriteMutInt for [u8] {}

#[cfg(feature = "chrono")]
use crate::chrono::Off64ReadChrono;
#[cfg(feature = "chrono")]
use crate::chrono::Off64WriteMutChrono;

#[cfg(feature = "chrono")]
impl<'a> Off64ReadChrono<'a, &'a [u8]> for [u8] {}
#[cfg(feature = "chrono")]
impl Off64WriteMutChrono for [u8] {}
