use crate::int::Off64ReadInt;
use crate::int::Off64WriteInt;
use crate::usz;
use crate::Off64Read;
use crate::Off64Write;
use std::fs::File;
use std::os::unix::prelude::FileExt;

impl<'a> Off64Read<'a, Vec<u8>> for File {
  fn read_at(&'a self, offset: u64, len: u64) -> Vec<u8> {
    let mut buf = vec![0u8; usz!(len)];
    self.read_exact_at(&mut buf, offset).unwrap();
    buf
  }
}

impl Off64Write<&[u8]> for File {
  fn write_at(&self, offset: u64, value: &[u8]) -> () {
    self.write_all_at(value, offset).unwrap();
  }
}

impl<'a> Off64ReadInt<'a, Vec<u8>> for File {}
impl Off64WriteInt for File {}

#[cfg(feature = "chrono")]
use crate::chrono::Off64ReadChrono;
#[cfg(feature = "chrono")]
use crate::chrono::Off64WriteChrono;

#[cfg(feature = "chrono")]
impl<'a> Off64ReadChrono<'a, Vec<u8>> for File {}
#[cfg(feature = "chrono")]
impl Off64WriteChrono for File {}
