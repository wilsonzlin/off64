use crate::int::*;
use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
pub trait Off64ReadChrono<'a, T: 'a + AsRef<[u8]>>: Off64ReadInt<'a, T> {
  fn read_timestamp_be_at(&'a self, offset: u64) -> DateTime<Utc> {
    let sec = self.read_i64_be_at(offset);
    Utc.timestamp_millis_opt(sec * 1000).unwrap()
  }

  fn read_timestamp_millis_be_at(&'a self, offset: u64) -> DateTime<Utc> {
    let ms = self.read_i64_be_at(offset);
    Utc.timestamp_millis_opt(ms).unwrap()
  }

  fn read_timestamp_le_at(&'a self, offset: u64) -> DateTime<Utc> {
    let sec = self.read_i64_le_at(offset);
    Utc.timestamp_millis_opt(sec * 1000).unwrap()
  }

  fn read_timestamp_millis_le_at(&'a self, offset: u64) -> DateTime<Utc> {
    let ms = self.read_i64_le_at(offset);
    Utc.timestamp_millis_opt(ms).unwrap()
  }
}
#[async_trait::async_trait]
pub trait Off64AsyncReadChrono<'a, T: 'a + AsRef<[u8]>>: Off64AsyncReadInt<'a, T> {
  async fn read_timestamp_be_at(&'a self, offset: u64) -> DateTime<Utc> {
    let sec = self.read_i64_be_at(offset).await;
    Utc.timestamp_millis_opt(sec * 1000).unwrap()
  }

  async fn read_timestamp_millis_be_at(&'a self, offset: u64) -> DateTime<Utc> {
    let ms = self.read_i64_be_at(offset).await;
    Utc.timestamp_millis_opt(ms).unwrap()
  }

  async fn read_timestamp_le_at(&'a self, offset: u64) -> DateTime<Utc> {
    let sec = self.read_i64_le_at(offset).await;
    Utc.timestamp_millis_opt(sec * 1000).unwrap()
  }

  async fn read_timestamp_millis_le_at(&'a self, offset: u64) -> DateTime<Utc> {
    let ms = self.read_i64_le_at(offset).await;
    Utc.timestamp_millis_opt(ms).unwrap()
  }
}
#[async_trait::async_trait]
pub trait Off64AsyncWriteMutChrono: Off64AsyncWriteMutInt {
  async fn write_timestamp_be_at(&mut self, offset: u64, value: DateTime<Utc>) {
    let sec = value.timestamp();
    self.write_i64_be_at(offset, sec).await;
  }

  async fn write_timestamp_millis_be_at(&mut self, offset: u64, value: DateTime<Utc>) {
    let ms = value.timestamp_millis();
    self.write_i64_be_at(offset, ms).await;
  }

  async fn write_timestamp_le_at(&mut self, offset: u64, value: DateTime<Utc>) {
    let sec = value.timestamp();
    self.write_i64_le_at(offset, sec).await;
  }

  async fn write_timestamp_millis_le_at(&mut self, offset: u64, value: DateTime<Utc>) {
    let ms = value.timestamp_millis();
    self.write_i64_le_at(offset, ms).await;
  }
}
#[async_trait::async_trait]
pub trait Off64AsyncWriteChrono: Off64AsyncWriteInt {
  async fn write_timestamp_be_at(&self, offset: u64, value: DateTime<Utc>) {
    let sec = value.timestamp();
    self.write_i64_be_at(offset, sec).await;
  }

  async fn write_timestamp_millis_be_at(&self, offset: u64, value: DateTime<Utc>) {
    let ms = value.timestamp_millis();
    self.write_i64_be_at(offset, ms).await;
  }

  async fn write_timestamp_le_at(&self, offset: u64, value: DateTime<Utc>) {
    let sec = value.timestamp();
    self.write_i64_le_at(offset, sec).await;
  }

  async fn write_timestamp_millis_le_at(&self, offset: u64, value: DateTime<Utc>) {
    let ms = value.timestamp_millis();
    self.write_i64_le_at(offset, ms).await;
  }
}
pub trait Off64WriteMutChrono: Off64WriteMutInt {
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
pub trait Off64WriteChrono: Off64WriteInt {
  fn write_timestamp_be_at(&self, offset: u64, value: DateTime<Utc>) {
    let sec = value.timestamp();
    self.write_i64_be_at(offset, sec);
  }

  fn write_timestamp_millis_be_at(&self, offset: u64, value: DateTime<Utc>) {
    let ms = value.timestamp_millis();
    self.write_i64_be_at(offset, ms);
  }

  fn write_timestamp_le_at(&self, offset: u64, value: DateTime<Utc>) {
    let sec = value.timestamp();
    self.write_i64_le_at(offset, sec);
  }

  fn write_timestamp_millis_le_at(&self, offset: u64, value: DateTime<Utc>) {
    let ms = value.timestamp_millis();
    self.write_i64_le_at(offset, ms);
  }
}
