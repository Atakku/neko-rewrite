// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use std::hash::Hasher;

// https://docs.rs/http/0.2.5/src/http/extensions.rs.html#8-28
// With TypeIds as keys, there's no need to hash them. They are already hashes
// themselves, coming from the compiler. The IdHasher just holds the u64 of
// the TypeId, and then returns it, instead of doing any bit fiddling.

#[derive(Default)]
pub struct IdHasher(u64);

impl Hasher for IdHasher {
  fn write(&mut self, _: &[u8]) {
    unreachable!("TypeId calls write_u64");
  }

  #[inline]
  fn write_u64(&mut self, id: u64) {
    self.0 = id;
  }

  #[inline]
  fn finish(&self) -> u64 {
    self.0
  }
}
