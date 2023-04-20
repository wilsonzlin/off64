use crate::int::Off64Int;
use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;

pub trait Off64Chrono: Off64Int {
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
