use async_trait::async_trait;

#[cfg(feature = "chrono")]
pub mod chrono;
pub mod file;
pub mod int;
pub mod range;
pub mod slice;
#[cfg(test)]
pub mod test;

pub trait Off64Read<'a, T: 'a + AsRef<[u8]>> {
  fn read_at(&'a self, offset: u64, len: u64) -> T;
}

#[async_trait]
pub trait Off64AsyncRead<'a, T: 'a + AsRef<[u8]>> {
  async fn read_at(&self, offset: u64, len: u64) -> T;
}

pub trait Off64Write<T: AsRef<[u8]>> {
  fn write_at(&self, offset: u64, value: T) -> ();
}

pub trait Off64WriteMut<T: AsRef<[u8]>> {
  fn write_at(&mut self, offset: u64, value: T) -> ();
}

#[async_trait]
pub trait Off64AsyncWrite<T: AsRef<[u8]>> {
  async fn write_at(&self, offset: u64, value: T) -> ();
}

#[async_trait]
pub trait Off64AsyncWriteMut<T: AsRef<[u8]>> {
  async fn write_at(&mut self, offset: u64, value: T) -> ();
}
