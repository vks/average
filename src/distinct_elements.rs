use arrayvec::ArrayVec;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rand::distributions::Standard;


/// Estimate the number of distinct elements of a sequence of numbers.
///
/// This uses the unbiased CVM algorithm by Chakraborty, Vinodchandran, and Meel [1].
///
/// [1]: https://cs.stanford.edu/~knuth/papers/cvm-note.pdf
#[derive(Debug, Clone)]
pub struct DistinctElements<const N: usize> {
    // Ideally, this should use a treap instead.
    buffer: ArrayVec<(u64, f64), N>,
    sampling_rate: f64,
    rng: SmallRng,
}

impl<const N: usize> DistinctElements<N> {
    /// Create a new estimator.
    pub fn new() -> Self {
        Self {
            buffer: ArrayVec::new(),
            sampling_rate: 1.0,
            rng: SmallRng::from_entropy(),
        }
    }

    /// Create a new estimator with a given RNG.
    pub fn from_rng(rng: SmallRng) -> Self {
        Self {
            buffer: ArrayVec::new(),
            sampling_rate: 1.0,
            rng,
        }
    }

    /// Add an element to the sequence of numbers.
    pub fn add(&mut self, a: u64) {
        self.remove(a);

        // Maybe put `a` in the buffer.
        let u: f64 = self.rng.sample(Standard);
        if u >= self.sampling_rate {
            return;
        }
        if self.buffer.len() < N {
            self.buffer.push((a, u));
            return;
        }

        // Maybe swap `a` into the buffer.
        let (ap, up) = self.find_max();
        if u > up {
            self.sampling_rate = u;
        } else {
            self.remove(ap);
            self.buffer.push((a, u));
            self.sampling_rate = up;
        }
    }

    /// Remove an element from the buffer.
    fn remove(&mut self, a: u64) {
        self.buffer.retain(|&mut (x, _)| x != a)
    }

    /// Find the element with the maximal sampling rate.
    fn find_max(&self) -> (u64, f64) {
        assert!(!self.buffer.is_empty());
        self.buffer.iter().copied().max_by(|(_, u1), (_, u2)| u1.partial_cmp(u2).unwrap()).unwrap()
    }

    /// Estimate the number of distinct elements in the sequence.
    pub fn estimate(&self) -> f64 {
        self.buffer.len() as f64 / self.sampling_rate
    }
}

impl<const N: usize> Default for DistinctElements<N> {
    fn default() -> Self {
        DistinctElements::new()
    }
}

impl<const N: usize> FromIterator<u64> for DistinctElements<N> {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        let mut de = DistinctElements::new();
        for a in iter {
            de.add(a);
        }
        de
    }
}

impl<'a, const N: usize> FromIterator<&'a u64> for DistinctElements<N> {
    fn from_iter<I: IntoIterator<Item = &'a u64>>(iter: I) -> Self {
        let mut de = DistinctElements::new();
        for &a in iter {
            de.add(a);
        }
        de
    }
}

impl<const N: usize> Extend<u64> for DistinctElements<N> {
    fn extend<I: IntoIterator<Item = u64>>(&mut self, iter: I) {
        for a in iter {
            self.add(a);
        }
    }
}

impl<'a, const N: usize> Extend<&'a u64> for DistinctElements<N> {
    fn extend<I: IntoIterator<Item = &'a u64>>(&mut self, iter: I) {
        for &a in iter {
            self.add(a);
        }
    }
}