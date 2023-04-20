use async_trait::async_trait;

#[cfg(feature = "chrono")]
pub mod chrono;
pub mod file;
pub mod int;
pub mod range;
pub mod slice;
#[cfg(test)]
pub mod test;

// The read trait method has a generic return type so the implementer can return borrowed (e.g. `&[u8]`) or owned (e.g. `Vec<u8>`) value depending on their requirements, and still be able to use all Off64 extension/helper traits and methods without any overhead.

pub trait Off64Read<'a, T: 'a + AsRef<[u8]>> {
  fn read_at(&'a self, offset: u64, len: u64) -> T;
}

#[async_trait]
pub trait Off64AsyncRead<'a, T: 'a + AsRef<[u8]>> {
  async fn read_at(&self, offset: u64, len: u64) -> T;
}

// The write traits do not take a generic `AsRef<[u8]>` value as the only (main) users of the trait method, the various Off64 extension/helper traits and methods, only ever write slices, so it would not be useful. If the implementation requires an owned value, clone the slice into a Vec; there's no *unnecessary* overhead as the value was not originally in the heap, so it's not a pointless double allocation. If a `write_at` method that takes an owned value is desired (e.g. to signal that an owned copy is required/preferred to avoid cloning), then just implement it as a regular method; Off64 does not require such a method and won't use it anyway, and other callers will simply call the method directly or have their own traits.

pub trait Off64Write {
  fn write_at(&self, offset: u64, value: &[u8]) -> ();
}

pub trait Off64WriteMut {
  fn write_at(&mut self, offset: u64, value: &[u8]) -> ();
}

#[async_trait]
pub trait Off64AsyncWrite {
  async fn write_at(&self, offset: u64, value: &[u8]) -> ();
}

#[async_trait]
pub trait Off64AsyncWriteMut {
  async fn write_at(&mut self, offset: u64, value: &[u8]) -> ();
}
