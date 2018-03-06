/// Define a histogram with a number of bins known at compile time.
///
/// ```
/// # extern crate core;
/// # #[macro_use] extern crate average;
/// # fn main() {
/// use average::Histogram;
///
/// define_histogram!(Histogram10, 10);
/// let mut h = Histogram10::with_const_width(0., 100.);
/// for i in 0..100 {
///     h.add(i as f64).unwrap();
/// }
/// assert_eq!(h.bins(), &[10, 10, 10, 10, 10, 10, 10, 10, 10, 10]);
/// # }
/// ```
#[macro_export]
macro_rules! define_histogram {
    ($name:ident, $LEN:expr) => (
        /// The number of bins of the histogram.
        const LEN: usize = $LEN;

        /// A histogram with a number of bins known at compile time.
        #[derive(Debug, Clone)]
        pub struct $name {
            range: [f64; LEN + 1],
            bin: [u64; LEN],
        }

        impl $name {
            /// Construct a histogram with constant bin width.
            #[inline]
            pub fn with_const_width(start: f64, end: f64) -> Self {
                let step = (end - start) / (LEN as f64);
                let mut range = [0.; LEN + 1];
                for (i, r) in range.iter_mut().enumerate() {
                    *r = step * (i as f64);
                }

                Self {
                    range,
                    bin: [0; LEN],
                }
            }

            /// Construct a histogram from given ranges.
            ///
            /// The ranges are given by an iterator of floats where neighboring
            /// pairs `(a, b)` define a bin for all `x` where `a <= x < b`.
            ///
            /// Fails if the iterator is too short (less than `n + 1` where `n`
            /// is the number of bins), is not sorted or contains `nan`. `inf`
            /// and empty ranges are allowed.
            #[inline]
            pub fn from_ranges<T>(ranges: T) -> Result<Self, ()>
                where T: IntoIterator<Item = f64>
            {
                let mut range = [0.; LEN + 1];
                let mut last_i = 0;
                for (i, r) in ranges.into_iter().enumerate() {
                    if i > LEN {
                        break;
                    }
                    if r.is_nan() {
                        return Err(());
                    }
                    if i > 0 && range[i - 1] > r {
                        return Err(());
                    }
                    range[i] = r;
                    last_i = i;
                }
                if last_i != LEN {
                    return Err(());
                }
                Ok(Self {
                    range,
                    bin: [0; LEN],
                })
            }

            /// Add a sample to the histogram.
            ///
            /// Fails if the sample is out of range of the histogram.
            #[inline]
            pub fn add(&mut self, x: f64) -> Result<(), ()> {
                // We made sure our ranges are valid at construction, so we can
                // safely unwrap.
                match self.range.binary_search_by(|p| p.partial_cmp(&x).unwrap()) {
                    Ok(i) if i < LEN => {
                        self.bin[i] += 1;
                    },
                    Err(i) if i > 0 && i < LEN + 1 => {
                        self.bin[i - 1] += 1;
                    },
                    _ => {
                        return Err(());
                    },
                }
                Ok(())
            }

            /// Return the ranges of the histogram.
            #[inline]
            pub fn ranges(&self) -> &[f64] {
                &self.range as &[f64]
            }

            /// Return an iterator over the bins and corresponding ranges:
            /// `((lower, upper), count)`
            #[inline]
            pub fn iter(&self) -> IterHistogram {
                self.into_iter()
            }

            /// Reset all bins to zero.
            #[inline]
            pub fn reset(&mut self) {
                self.bin = [0; LEN];
            }

            /// Return the lower range limit.
            #[inline]
            pub fn min(&self) -> f64 {
                self.range[0]
            }

            /// Return the upper range limit.
            #[inline]
            pub fn max(&self) -> f64 {
                self.range[LEN]
            }
        }

        /// Iterate over all `(range, count)` pairs in the histogram.
        pub struct IterHistogram<'a> {
            remaining_bin: &'a [u64],
            remaining_range: &'a [f64],
        }

        impl<'a> ::core::iter::Iterator for IterHistogram<'a> {
            type Item = ((f64, f64), u64);
            fn next(&mut self) -> Option<((f64, f64), u64)> {
                if let Some((&bin, rest)) = self.remaining_bin.split_first() {
                    let left = self.remaining_range[0];
                    let right = self.remaining_range[1];
                    self.remaining_bin = rest;
                    self.remaining_range = &self.remaining_range[1..];
                    return Some(((left, right), bin));
                }
                None
            }
        }

        impl<'a> ::core::iter::IntoIterator for &'a $name {
            type Item = ((f64, f64), u64);
            type IntoIter = IterHistogram<'a>;
            fn into_iter(self) -> IterHistogram<'a> {
                IterHistogram {
                    remaining_bin: self.bins(),
                    remaining_range: self.ranges(),
                }
            }
        }

        impl $crate::Histogram for $name {
            #[inline]
            fn bins(&self) -> &[u64] {
                &self.bin as &[u64]
            }
        }
    );
}
