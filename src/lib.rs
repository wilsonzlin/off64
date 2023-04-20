#[cfg(test)]
pub mod test;

#[cfg(feature = "chrono")]
use chrono::DateTime;
#[cfg(feature = "chrono")]
use chrono::TimeZone;
#[cfg(feature = "chrono")]
use chrono::Utc;
use std::ops::Bound;
use std::ops::RangeBounds;

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

include!(concat!(env!("OUT_DIR"), "/ints.rs"));

pub trait Off64Slice {
  fn read_slice_at(&self, offset: u64, len: u64) -> &[u8];
  // For when you want to read up to a certain offset instead of a length.
  fn read_slice_at_range<R: RangeBounds<u64>>(&self, range: R) -> &[u8];
  fn write_slice_at<'v>(&mut self, offset: u64, value: &'v [u8]) -> ();
}

impl Off64Slice for [u8] {
  fn read_slice_at(&self, offset: u64, len: u64) -> &[u8] {
    &self[usz!(offset)..usz!(offset + len)]
  }

  fn read_slice_at_range<R: RangeBounds<u64>>(&self, range: R) -> &[u8] {
    let start = match range.start_bound() {
      Bound::Included(&n) => usz!(n),
      Bound::Excluded(&n) => usz!(n + 1),
      Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
      Bound::Included(&n) => usz!(n + 1),
      Bound::Excluded(&n) => usz!(n),
      Bound::Unbounded => self.len(),
    };
    &self[start..end]
  }

  fn write_slice_at<'v>(&mut self, offset: u64, value: &'v [u8]) {
    self[usz!(offset)..usz!(offset) + value.len()].copy_from_slice(value);
  }
}

#[cfg(feature = "chrono")]
pub trait Off64Chrono {
  fn read_timestamp_be_at(&self, offset: u64) -> DateTime<Utc>;
  fn read_timestamp_millis_be_at(&self, offset: u64) -> DateTime<Utc>;
  fn read_timestamp_le_at(&self, offset: u64) -> DateTime<Utc>;
  fn read_timestamp_millis_le_at(&self, offset: u64) -> DateTime<Utc>;
  fn write_timestamp_be_at(&mut self, offset: u64, value: DateTime<Utc>) -> ();
  fn write_timestamp_millis_be_at(&mut self, offset: u64, value: DateTime<Utc>) -> ();
  fn write_timestamp_le_at(&mut self, offset: u64, value: DateTime<Utc>) -> ();
  fn write_timestamp_millis_le_at(&mut self, offset: u64, value: DateTime<Utc>) -> ();
}

#[cfg(feature = "chrono")]
impl Off64Chrono for [u8] {
  fn read_timestamp_be_at(&self, offset: u64) -> DateTime<Utc> {
    let sec = self.read_i64_be_at(offset);
    Utc.timestamp_millis_opt(sec * 1000).unwrap()
  }

  fn read_timestamp_millis_be_at(&self, offset: u64) -> DateTime<Utc> {
    let ms = self.read_i64_be_at(offset);
    Utc.timestamp_millis_opt(ms).unwrap()
  }

  fn read_timestamp_le_at(&self, offset: u64) -> DateTime<Utc> {
    let sec = self.read_i64_le_at(offset);
    Utc.timestamp_millis_opt(sec * 1000).unwrap()
  }

  fn read_timestamp_millis_le_at(&self, offset: u64) -> DateTime<Utc> {
    let ms = self.read_i64_le_at(offset);
    Utc.timestamp_millis_opt(ms).unwrap()
  }

  fn write_timestamp_be_at(&mut self, offset: u64, value: DateTime<Utc>) {
    let sec = value.timestamp();
    self.write_i64_be_at(offset, sec);
  }

  fn write_timestamp_millis_be_at(&mut self, offset: u64, value: DateTime<Utc>) {
    let ms = value.timestamp_millis();
    self.write_i64_be_at(offset, ms);
  }

  fn write_timestamp_le_at(&mut self, offset: u64, value: DateTime<Utc>) {
    let sec = value.timestamp();
    self.write_i64_le_at(offset, sec);
  }

  fn write_timestamp_millis_le_at(&mut self, offset: u64, value: DateTime<Utc>) {
    let ms = value.timestamp_millis();
    self.write_i64_le_at(offset, ms);
  }
}
