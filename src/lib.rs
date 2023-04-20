#[cfg(feature = "chrono")]
pub mod chrono;
pub mod int;
pub mod range;
pub mod slice;
#[cfg(test)]
pub mod test;

use async_trait::async_trait;

pub trait Off64 {
  fn read_at(&self, offset: u64, len: u64) -> &[u8];
  fn write_at<'v>(&mut self, offset: u64, value: &'v [u8]) -> ();
}

// Some types don't support reading and writing with borrowed data, like files.
pub trait Off64Owned {
  fn read_at(&self, offset: u64, len: u64) -> Vec<u8>;
  fn write_at<'v>(&mut self, offset: u64, value: Vec<u8>) -> ();
}

#[async_trait]
pub trait Off64Async {
  async fn read_at(&self, offset: u64, len: u64) -> &[u8];
  async fn write_at<'v>(&mut self, offset: u64, value: &'v [u8]) -> ();
}

// Some types don't support reading and writing with borrowed data, like files.
#[async_trait]
pub trait Off64AsyncOwned {
  async fn read_at(&self, offset: u64, len: u64) -> Vec<u8>;
  async fn write_at<'v>(&mut self, offset: u64, value: Vec<u8>) -> ();
}
