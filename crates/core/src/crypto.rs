use rand::rngs::OsRng;
use rand::RngCore;

/// Cryptographically secure random digit generator
/// Uses OS-level CSPRNG with rejection sampling to eliminate modulo bias
pub struct SecureRng {
    rng: OsRng,
}

impl SecureRng {
    pub fn new() -> Self {
        Self { rng: OsRng }
    }

    /// Generate a single digit (0-9) with uniform distribution
    /// Uses rejection sampling to eliminate modulo bias
    pub fn digit(&mut self) -> u8 {
        loop {
            let byte = (self.rng.next_u32() & 0xFF) as u8;
            // Reject values >= 250 to ensure uniform distribution over 0-9
            if byte < 250 {
                return byte % 10;
            }
        }
    }

    /// Generate a digit in range [lo, hi] inclusive
    pub fn digit_range(&mut self, lo: u8, hi: u8) -> u8 {
        debug_assert!(lo <= hi && hi <= 9);
        let range = (hi - lo + 1) as u32;
        loop {
            let val = self.rng.next_u32();
            // Rejection sampling for uniform distribution
            let max_valid = (u32::MAX / range) * range;
            if val < max_valid {
                return lo + (val % range) as u8;
            }
        }
    }

    /// Generate a non-zero digit (1-9)
    pub fn nonzero_digit(&mut self) -> u8 {
        self.digit_range(1, 9)
    }

    /// Pick a random element from a slice
    pub fn pick_from<T: Clone>(&mut self, items: &[T]) -> T {
        let idx = self.rng.next_u32() as usize % items.len();
        items[idx].clone()
    }

    /// Generate a random u32 in [lo, hi) range
    pub fn range_u32(&mut self, lo: u32, hi: u32) -> u32 {
        let range = hi - lo;
        if range == 0 {
            return lo;
        }
        lo + (self.rng.next_u32() % range)
    }
}

impl Default for SecureRng {
    fn default() -> Self {
        Self::new()
    }
}
