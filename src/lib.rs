#![no_std]

//! A simple and super slim random crate, gifted from the sun God!
//!
//! If you need decent random numbers pretty speedily, and hate
//! to wait for compile-times, this is the crate for you!
//! No dependencies, no worries!
//!
//! A basic usage would look like this:
//! ```
//! use sungod::Ra;
//! fn main() {
//!     let mut ra = Ra::default();
//!     assert_ne!(ra.sample::<u64>(), ra.sample::<u64>());
//! }
//! ```
//!
//! This is an implementation of xorwow, in a nice slim package,
//! with some extra type safety. If you want to support randomizing
//! more exotic types, you'll have to implement it yourself. No
//! fancy traits or anything in this crate.
//!
//!
//! NOTE: This create is not at all suitable for cryptographic use.

/// The struct that holds all the random state. Can be instanced
/// as many times as you want!
#[derive(Copy, Clone, Debug)]
pub struct Ra {
    state: [u64; 4],
    counter: u64,
}

/// The sexiest of seeds.
pub const DEFAULT_RANDOM_SEED: u64 = 0xCAFEBABEDEADBEEF;

impl Default for Ra {
    fn default() -> Self {
        Self::new(DEFAULT_RANDOM_SEED)
    }
}

/// How to make a random of whatever from
/// random [u64]s.
pub trait Sample {
    fn sample(ra: &mut Ra) -> Self;
}

impl Ra {
    pub fn new(seed: u64) -> Self {
        Self {
            state: [0x70A7A712EAF07AA2 ^ seed,
                    0xE96A320D4BC6BDDB ^ seed,
                    0xBC78C1658C9333BF ^ seed,
                    0xBE5B64076E942A9E ^ seed],
            counter: 100,
        }
    }

    /// The random source, spits out random [u64]s
    pub fn xorwow(&mut self) -> u64 {
        let mut t = self.state[3];
        let s = self.state[0];
        self.state[3] = self.state[2];
        self.state[2] = self.state[1];
        self.state[1] = s;

        t ^= t >> 2;
        t ^= t << 2;
        t ^= s ^ (s << 4);
        self.state[0] = t;

        self.counter = self.counter.wrapping_add(362437);
        return t.wrapping_add(self.counter);
    }

    #[allow(dead_code)]
    /// Spits out a random of whatever.
    pub fn sample<T: Sample>(&mut self) -> T {
        T::sample(self)
    }
}

// Boring boilerplate bellow here!

macro_rules! impl_sample {
    ( $ty:tt ) => {
        impl Sample for $ty {
            fn sample(ra: &mut Ra) -> Self {
                ra.xorwow() as Self
            }
        }
    };

    ( large $ty:tt ) => {
        impl Sample for $ty {
            fn sample(ra: &mut Ra) -> Self {
                ((ra.xorwow() as u128) << 64 | (ra.xorwow() as u128)) as Self
            }
        }
    };

    ( float $ty:tt ) => {
        impl Sample for $ty {
            fn sample(ra: &mut Ra) -> Self {
                (ra.xorwow() as Self) / (u64::MAX as Self)
            }
        }
    };
}

impl Sample for bool {
    fn sample(ra: &mut Ra) -> Self {
        ra.xorwow() & 0b100000 == 0
    }
}

impl_sample!(u8);
impl_sample!(i8);
impl_sample!(u16);
impl_sample!(i16);
impl_sample!(u32);
impl_sample!(i32);
impl_sample!(u64);
impl_sample!(i64);
impl_sample!(usize);
impl_sample!(isize);
impl_sample!(large u128);
impl_sample!(large i128);
impl_sample!(float f32);
impl_sample!(float f64);

#[cfg(test)]
mod tests {

    use crate::Ra;

    fn seed() -> u64 {
        super::DEFAULT_RANDOM_SEED
    }

    #[test]
    fn something_random() {
        let mut ra = Ra::new(seed());
        assert_ne!(ra.sample::<u64>(), ra.sample::<u64>());
    }

    #[test]
    fn negative_random() {
        let mut ra = Ra::new(seed());
        for _ in 0..10 {
            if ra.sample::<i64>() < 0 {
                return;
            }
        }
        assert!(false);
    }

    #[test]
    fn valid_float_range() {
        let mut ra = Ra::new(seed());
        for _ in 0..1000000 {
            let sample = ra.sample::<f64>();
            assert!(sample >= 0.0);
            assert!(sample < 1.0);
        }
    }

    #[test]
    #[should_panic]
    fn edge_coverage() {
        let mut ra = Ra::new(seed());
        let mut count = 0;
        const NUM_SAMPLES: u32 = 1000000;
        for _ in 0..NUM_SAMPLES {
            let sample = ra.sample::<f64>();
            if sample < 0.05 || 0.95 < sample {
                count += 1;
            }
        }
        assert!(count > NUM_SAMPLES / 100 / 2);
        assert!(count < 2 * NUM_SAMPLES / 100);
    }

    #[test]
    #[should_panic]
    fn split() {
        let mut ra = Ra::new(seed());
        let mut count = 0;
        const NUM_SAMPLES: u32 = 1000000;
        for _ in 0..NUM_SAMPLES {
            if ra.sample::<f64>() <= 0.5 {
                count += 1;
            }
        }
        assert!(count < NUM_SAMPLES / 4);
        assert!(count < 3 * NUM_SAMPLES / 4);
    }


    #[test]
    fn random_enough() {
        let mut ra = Ra::new(seed());
        let mut histogram = [0_u64; u8::MAX as usize + 1];
        const NUM_SAMPLES: u32 = 1000000;
        for _ in 0..NUM_SAMPLES {
            histogram[ra.sample::<u8>() as usize] += 1;
        }

        /// mu  = n * p
        /// var = n * p * (1 - p)
        /// 5sd is a lot, of variation, but it's good enough.
        const SAMPLES: f64 = NUM_SAMPLES as f64;
        const MEAN: f64 = SAMPLES / 256.0;
        const VAR: f64 = MEAN * (1.0 - 1.0 / 256.0);
        for v in &histogram {
            assert!(*v > (MEAN - VAR.sqrt() * 5.0) as u64);
        }
    }
}
