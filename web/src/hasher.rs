/// The hasher used to compare layouts.
#[derive(Debug)]
pub struct Hasher(std::collections::hash_map::DefaultHasher);

impl Default for Hasher {
    fn default() -> Self {
        Hasher(std::collections::hash_map::DefaultHasher::new())
    }
}

impl core::hash::Hasher for Hasher {
    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }

    fn finish(&self) -> u64 {
        self.0.finish()
    }
}
