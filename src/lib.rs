#[cfg(feature = "chrono")]
use chrono::DateTime;
#[cfg(feature = "chrono")]
use chrono::TimeZone;
#[cfg(feature = "chrono")]
use chrono::Utc;
use std::mem::size_of;
use std::ops::Bound;
use std::ops::RangeBounds;

// A short convenient macro for converting to usize without using the unsafe `as` operator.
#[macro_export]
macro_rules! usz {
  ($v:expr) => {
    usize::try_from($v).unwrap()
  };
}

pub trait Off64 {
  fn read_i16_be_at(&self, offset: u64) -> i16;
  fn read_i32_be_at(&self, offset: u64) -> i32;
  fn read_i64_be_at(&self, offset: u64) -> i64;
  fn read_u16_be_at(&self, offset: u64) -> u16;
  fn read_u32_be_at(&self, offset: u64) -> u32;
  fn read_u64_be_at(&self, offset: u64) -> u64;
  #[cfg(feature = "chrono")]
  fn read_timestamp_be_at(&self, offset: u64) -> DateTime<Utc>;
  #[cfg(feature = "chrono")]
  fn read_timestamp_millis_be_at(&self, offset: u64) -> DateTime<Utc>;
  fn read_i16_le_at(&self, offset: u64) -> i16;
  fn read_i32_le_at(&self, offset: u64) -> i32;
  fn read_i64_le_at(&self, offset: u64) -> i64;
  fn read_u16_le_at(&self, offset: u64) -> u16;
  fn read_u32_le_at(&self, offset: u64) -> u32;
  fn read_u64_le_at(&self, offset: u64) -> u64;
  fn read_slice_at(&self, offset: u64, len: u64) -> &[u8];
  #[cfg(feature = "chrono")]
  fn read_timestamp_le_at(&self, offset: u64) -> DateTime<Utc>;
  #[cfg(feature = "chrono")]
  fn read_timestamp_millis_le_at(&self, offset: u64) -> DateTime<Utc>;
  // For when you want to read up to a certain offset instead of a length.
  fn read_slice_at_range<R: RangeBounds<u64>>(&self, range: R) -> &[u8];
  fn write_i16_be_at(&mut self, offset: u64, value: i16) -> ();
  fn write_i32_be_at(&mut self, offset: u64, value: i32) -> ();
  fn write_i64_be_at(&mut self, offset: u64, value: i64) -> ();
  fn write_u16_be_at(&mut self, offset: u64, value: u16) -> ();
  fn write_u32_be_at(&mut self, offset: u64, value: u32) -> ();
  fn write_u64_be_at(&mut self, offset: u64, value: u64) -> ();
  #[cfg(feature = "chrono")]
  fn write_timestamp_be_at(&mut self, offset: u64, value: DateTime<Utc>) -> ();
  #[cfg(feature = "chrono")]
  fn write_timestamp_millis_be_at(&mut self, offset: u64, value: DateTime<Utc>) -> ();
  fn write_i16_le_at(&mut self, offset: u64, value: i16) -> ();
  fn write_i32_le_at(&mut self, offset: u64, value: i32) -> ();
  fn write_i64_le_at(&mut self, offset: u64, value: i64) -> ();
  fn write_u16_le_at(&mut self, offset: u64, value: u16) -> ();
  fn write_u32_le_at(&mut self, offset: u64, value: u32) -> ();
  fn write_u64_le_at(&mut self, offset: u64, value: u64) -> ();
  #[cfg(feature = "chrono")]
  fn write_timestamp_le_at(&mut self, offset: u64, value: DateTime<Utc>) -> ();
  #[cfg(feature = "chrono")]
  fn write_timestamp_millis_le_at(&mut self, offset: u64, value: DateTime<Utc>) -> ();
  fn write_slice_at<'v>(&mut self, offset: u64, value: &'v [u8]) -> ();
}

impl Off64 for [u8] {
  fn read_i16_be_at(&self, offset: u64) -> i16 {
    let offset = usz!(offset);
    i16::from_be_bytes(self[offset..offset + size_of::<i16>()].try_into().unwrap())
  }

  fn read_i32_be_at(&self, offset: u64) -> i32 {
    let offset = usz!(offset);
    i32::from_be_bytes(self[offset..offset + size_of::<i32>()].try_into().unwrap())
  }

  fn read_i64_be_at(&self, offset: u64) -> i64 {
    let offset = usz!(offset);
    i64::from_be_bytes(self[offset..offset + size_of::<i64>()].try_into().unwrap())
  }

  fn read_u16_be_at(&self, offset: u64) -> u16 {
    let offset = usz!(offset);
    u16::from_be_bytes(self[offset..offset + size_of::<u16>()].try_into().unwrap())
  }

  fn read_u32_be_at(&self, offset: u64) -> u32 {
    let offset = usz!(offset);
    u32::from_be_bytes(self[offset..offset + size_of::<u32>()].try_into().unwrap())
  }

  fn read_u64_be_at(&self, offset: u64) -> u64 {
    let offset = usz!(offset);
    u64::from_be_bytes(self[offset..offset + size_of::<u64>()].try_into().unwrap())
  }

  #[cfg(feature = "chrono")]
  fn read_timestamp_be_at(&self, offset: u64) -> DateTime<Utc> {
    let sec = self.read_i64_be_at(offset);
    Utc.timestamp_millis_opt(sec * 1000).unwrap()
  }

  #[cfg(feature = "chrono")]
  fn read_timestamp_millis_be_at(&self, offset: u64) -> DateTime<Utc> {
    let ms = self.read_i64_be_at(offset);
    Utc.timestamp_millis_opt(ms).unwrap()
  }

  fn read_i16_le_at(&self, offset: u64) -> i16 {
    let offset = usz!(offset);
    i16::from_le_bytes(self[offset..offset + size_of::<i16>()].try_into().unwrap())
  }

  fn read_i32_le_at(&self, offset: u64) -> i32 {
    let offset = usz!(offset);
    i32::from_le_bytes(self[offset..offset + size_of::<i32>()].try_into().unwrap())
  }

  fn read_i64_le_at(&self, offset: u64) -> i64 {
    let offset = usz!(offset);
    i64::from_le_bytes(self[offset..offset + size_of::<i64>()].try_into().unwrap())
  }

  fn read_u16_le_at(&self, offset: u64) -> u16 {
    let offset = usz!(offset);
    u16::from_le_bytes(self[offset..offset + size_of::<u16>()].try_into().unwrap())
  }

  fn read_u32_le_at(&self, offset: u64) -> u32 {
    let offset = usz!(offset);
    u32::from_le_bytes(self[offset..offset + size_of::<u32>()].try_into().unwrap())
  }

  fn read_u64_le_at(&self, offset: u64) -> u64 {
    let offset = usz!(offset);
    u64::from_le_bytes(self[offset..offset + size_of::<u64>()].try_into().unwrap())
  }

  #[cfg(feature = "chrono")]
  fn read_timestamp_le_at(&self, offset: u64) -> DateTime<Utc> {
    let sec = self.read_i64_le_at(offset);
    Utc.timestamp_millis_opt(sec * 1000).unwrap()
  }

  #[cfg(feature = "chrono")]
  fn read_timestamp_millis_le_at(&self, offset: u64) -> DateTime<Utc> {
    let ms = self.read_i64_le_at(offset);
    Utc.timestamp_millis_opt(ms).unwrap()
  }

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

  fn write_i16_be_at(&mut self, offset: u64, value: i16) {
    let offset = usz!(offset);
    self[offset..offset + size_of::<i16>()].copy_from_slice(&value.to_be_bytes());
  }

  fn write_i32_be_at(&mut self, offset: u64, value: i32) {
    let offset = usz!(offset);
    self[offset..offset + size_of::<i32>()].copy_from_slice(&value.to_be_bytes());
  }

  fn write_i64_be_at(&mut self, offset: u64, value: i64) {
    let offset = usz!(offset);
    self[offset..offset + size_of::<i64>()].copy_from_slice(&value.to_be_bytes());
  }

  fn write_u16_be_at(&mut self, offset: u64, value: u16) {
    let offset = usz!(offset);
    self[offset..offset + size_of::<u16>()].copy_from_slice(&value.to_be_bytes());
  }

  fn write_u32_be_at(&mut self, offset: u64, value: u32) {
    let offset = usz!(offset);
    self[offset..offset + size_of::<u32>()].copy_from_slice(&value.to_be_bytes());
  }

  fn write_u64_be_at(&mut self, offset: u64, value: u64) {
    let offset = usz!(offset);
    self[offset..offset + size_of::<u64>()].copy_from_slice(&value.to_be_bytes());
  }

  #[cfg(feature = "chrono")]
  fn write_timestamp_be_at(&mut self, offset: u64, value: DateTime<Utc>) {
    let sec = value.timestamp();
    self.write_i64_be_at(offset, sec);
  }

  #[cfg(feature = "chrono")]
  fn write_timestamp_millis_be_at(&mut self, offset: u64, value: DateTime<Utc>) {
    let ms = value.timestamp_millis();
    self.write_i64_be_at(offset, ms);
  }

  fn write_i16_le_at(&mut self, offset: u64, value: i16) {
    let offset = usz!(offset);
    self[offset..offset + size_of::<i16>()].copy_from_slice(&value.to_le_bytes());
  }

  fn write_i32_le_at(&mut self, offset: u64, value: i32) {
    let offset = usz!(offset);
    self[offset..offset + size_of::<i32>()].copy_from_slice(&value.to_le_bytes());
  }

  fn write_i64_le_at(&mut self, offset: u64, value: i64) {
    let offset = usz!(offset);
    self[offset..offset + size_of::<i64>()].copy_from_slice(&value.to_le_bytes());
  }

  fn write_u16_le_at(&mut self, offset: u64, value: u16) {
    let offset = usz!(offset);
    self[offset..offset + size_of::<u16>()].copy_from_slice(&value.to_le_bytes());
  }

  fn write_u32_le_at(&mut self, offset: u64, value: u32) {
    let offset = usz!(offset);
    self[offset..offset + size_of::<u32>()].copy_from_slice(&value.to_le_bytes());
  }

  fn write_u64_le_at(&mut self, offset: u64, value: u64) {
    let offset = usz!(offset);
    self[offset..offset + size_of::<u64>()].copy_from_slice(&value.to_le_bytes());
  }

  #[cfg(feature = "chrono")]
  fn write_timestamp_le_at(&mut self, offset: u64, value: DateTime<Utc>) {
    let sec = value.timestamp();
    self.write_i64_le_at(offset, sec);
  }

  #[cfg(feature = "chrono")]
  fn write_timestamp_millis_le_at(&mut self, offset: u64, value: DateTime<Utc>) {
    let ms = value.timestamp_millis();
    self.write_i64_le_at(offset, ms);
  }

  fn write_slice_at<'v>(&mut self, offset: u64, value: &'v [u8]) {
    self[usz!(offset)..usz!(offset) + value.len()].copy_from_slice(value);
  }
}
