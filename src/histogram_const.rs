//! Histogram implementation via const generics.

/// Invalid ranges were specified for constructing the histogram.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidRangeError {
    /// The number of ranges is less than the number of bins + 1.
    NotEnoughRanges,
    /// The ranges are not sorted or `(low, high)` where `low` > `high` is
    /// encountered.
    NotSorted,
    /// A range contains `nan`.
    NaN,
}

/// A sample is out of range of the histogram.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SampleOutOfRangeError;

impl<const LEN: usize> ::core::fmt::Debug for Histogram<LEN>
where
    [u8; LEN + 1]: Sized,
{
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        formatter.write_str("Histogram {{ range: ")?;
        self.range[..].fmt(formatter)?;
        formatter.write_str(", bins: ")?;
        self.bin[..].fmt(formatter)?;
        formatter.write_str(" }}")
    }
}

impl<const LEN: usize> Histogram<LEN>
where
    [u8; LEN + 1]: Sized,
{
    /// Construct a histogram with constant bin width.
    #[inline]
    pub fn with_const_width(start: f64, end: f64) -> Self {
        let step = (end - start) / (LEN as f64);
        let mut range = [0.; LEN + 1];
        for (i, r) in range.iter_mut().enumerate() {
            *r = start + step * (i as f64);
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
    pub fn from_ranges<T>(ranges: T) -> Result<Self, InvalidRangeError>
    where
        T: IntoIterator<Item = f64>,
    {
        let mut range = [0.; LEN + 1];
        let mut last_i = 0;
        for (i, r) in ranges.into_iter().enumerate() {
            if i > LEN {
                break;
            }
            if r.is_nan() {
                return Err(InvalidRangeError::NaN);
            }
            if i > 0 && range[i - 1] > r {
                return Err(InvalidRangeError::NotSorted);
            }
            range[i] = r;
            last_i = i;
        }
        if last_i != LEN {
            return Err(InvalidRangeError::NotEnoughRanges);
        }
        Ok(Self {
            range,
            bin: [0; LEN],
        })
    }

    /// Find the index of the bin corresponding to the given sample.
    ///
    /// Fails if the sample is out of range of the histogram.
    #[inline]
    pub fn find(&self, x: f64) -> Result<usize, SampleOutOfRangeError> {
        // We made sure our ranges are valid at construction, so we can
        // safely unwrap.
        match self.range.binary_search_by(|p| p.partial_cmp(&x).unwrap()) {
            Ok(i) if i < LEN => Ok(i),
            Err(i) if i > 0 && i < LEN + 1 => Ok(i - 1),
            _ => Err(SampleOutOfRangeError),
        }
    }

    /// Add a sample to the histogram.
    ///
    /// Fails if the sample is out of range of the histogram.
    #[inline]
    pub fn add(&mut self, x: f64) -> Result<(), SampleOutOfRangeError> {
        if let Ok(i) = self.find(x) {
            self.bin[i] += 1;
            Ok(())
        } else {
            Err(SampleOutOfRangeError)
        }
    }

    /// Return the ranges of the histogram.
    #[inline]
    pub fn ranges(&self) -> &[f64] {
        &self.range[..]
    }

    /// Return an iterator over the bins and corresponding ranges:
    /// `((lower, upper), count)`
    #[inline]
    pub fn iter(&self) -> IterHistogram<'_> {
        self.into_iter()
    }

    /// Reset all bins to zero.
    #[inline]
    pub fn reset(&mut self) {
        self.bin = [0; LEN];
    }

    /// Return the lower range limit.
    ///
    /// (The corresponding bin might be empty.)
    #[inline]
    pub fn range_min(&self) -> f64 {
        self.range[0]
    }

    /// Return the upper range limit.
    ///
    /// (The corresponding bin might be empty.)
    #[inline]
    pub fn range_max(&self) -> f64 {
        self.range[LEN]
    }

    /// Return the bins of the histogram.
    #[inline]
    pub fn bins(&self) -> &[u64] {
        &self.bin[..]
    }

    /// Estimate the variance for the given bin.
    ///
    /// The square root of this estimates the error of the bin count.
    #[inline]
    pub fn variance(&self, bin: usize) -> f64 {
        let count = self.bins()[bin];
        let sum: u64 = self.bins().iter().sum();
        multinomial_variance(count as f64, 1. / (sum as f64))
    }

    /// Return an iterator over the bins normalized by the bin widths.
    #[inline]
    pub fn normalized_bins(&self) -> IterNormalized<<&Self as IntoIterator>::IntoIter> {
        IterNormalized {
            histogram_iter: self.into_iter(),
        }
    }

    /// Return an iterator over the bin widths.
    #[inline]
    pub fn widths(&self) -> IterWidths<<&Self as IntoIterator>::IntoIter> {
        IterWidths {
            histogram_iter: self.into_iter(),
        }
    }

    /// Return an iterator over the bin centers.
    #[inline]
    pub fn centers(&self) -> IterBinCenters<<&Self as IntoIterator>::IntoIter> {
        IterBinCenters {
            histogram_iter: self.into_iter(),
        }
    }

    /// Return an iterator over the bin variances.
    ///
    /// This is more efficient than calling `variance()` for each bin.
    #[inline]
    pub fn variances(&self) -> IterVariances<<&Self as IntoIterator>::IntoIter> {
        let sum: u64 = self.bins().iter().sum();
        IterVariances {
            histogram_iter: self.into_iter(),
            sum_inv: 1. / (sum as f64),
        }
    }
}

/// Iterate over all `(range, count)` pairs in the histogram.
#[derive(Clone, Debug)]
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

impl<'a, const LEN: usize> ::core::iter::IntoIterator for &'a Histogram<LEN>
where
    [u8; LEN + 1]: Sized,
{
    type Item = ((f64, f64), u64);
    type IntoIter = IterHistogram<'a>;
    fn into_iter(self) -> IterHistogram<'a> {
        IterHistogram {
            remaining_bin: self.bins(),
            remaining_range: self.ranges(),
        }
    }
}

impl<'a, const LEN: usize> ::core::ops::AddAssign<&'a Self> for Histogram<LEN>
where
    [u8; LEN + 1]: Sized,
{
    #[inline]
    fn add_assign(&mut self, other: &Self) {
        for (a, b) in self.range.iter().zip(other.range.iter()) {
            assert_eq!(a, b, "Both histograms must have the same ranges");
        }
        for (x, y) in self.bin.iter_mut().zip(other.bin.iter()) {
            *x += y;
        }
    }
}

impl<const LEN: usize> ::core::ops::MulAssign<u64> for Histogram<LEN>
where
    [u8; LEN + 1]: Sized,
{
    #[inline]
    fn mul_assign(&mut self, other: u64) {
        for x in &mut self.bin[..] {
            *x *= other;
        }
    }
}

impl<const LEN: usize> crate::Merge for Histogram<LEN>
where
    [u8; LEN + 1]: Sized,
{
    fn merge(&mut self, other: &Self) {
        assert_eq!(self.bin.len(), other.bin.len());
        for (a, b) in self.range.iter().zip(other.range.iter()) {
            assert_eq!(a, b, "Both histograms must have the same ranges");
        }
        for (a, b) in self.bin.iter_mut().zip(other.bin.iter()) {
            *a += *b;
        }
    }
}

/// A histogram with a number of bins known at compile time.
#[derive(Clone)]
pub struct Histogram<const LEN: usize>
where
    [u8; LEN + 1]: Sized,
{
    /// The ranges defining the bins of the histogram.
    range: [f64; LEN + 1],
    /// The bins of the histogram.
    bin: [u64; LEN],
}

/// Calculate the multinomial variance. Relevant for histograms.
#[inline(always)]
fn multinomial_variance(n: f64, n_tot_inv: f64) -> f64 {
    n * (1. - n * n_tot_inv)
}

/// Iterate over the bins normalized by bin width.
#[derive(Clone, Debug)]
pub struct IterNormalized<T>
where
    T: Iterator<Item = ((f64, f64), u64)>,
{
    histogram_iter: T,
}

impl<T> Iterator for IterNormalized<T>
where
    T: Iterator<Item = ((f64, f64), u64)>,
{
    type Item = f64;

    #[inline]
    fn next(&mut self) -> Option<f64> {
        self.histogram_iter
            .next()
            .map(|((a, b), count)| (count as f64) / (b - a))
    }
}

/// Iterate over the widths of the bins.
#[derive(Clone, Debug)]
pub struct IterWidths<T>
where
    T: Iterator<Item = ((f64, f64), u64)>,
{
    histogram_iter: T,
}

impl<T> Iterator for IterWidths<T>
where
    T: Iterator<Item = ((f64, f64), u64)>,
{
    type Item = f64;

    #[inline]
    fn next(&mut self) -> Option<f64> {
        self.histogram_iter.next().map(|((a, b), _)| b - a)
    }
}

/// Iterate over the bin centers.
#[derive(Clone, Debug)]
pub struct IterBinCenters<T>
where
    T: Iterator<Item = ((f64, f64), u64)>,
{
    histogram_iter: T,
}

impl<T> Iterator for IterBinCenters<T>
where
    T: Iterator<Item = ((f64, f64), u64)>,
{
    type Item = f64;

    #[inline]
    fn next(&mut self) -> Option<f64> {
        self.histogram_iter.next().map(|((a, b), _)| 0.5 * (a + b))
    }
}

/// Iterate over the variances.
#[derive(Clone, Debug)]
pub struct IterVariances<T>
where
    T: Iterator<Item = ((f64, f64), u64)>,
{
    histogram_iter: T,
    sum_inv: f64,
}

impl<T> Iterator for IterVariances<T>
where
    T: Iterator<Item = ((f64, f64), u64)>,
{
    type Item = f64;

    #[inline]
    fn next(&mut self) -> Option<f64> {
        self.histogram_iter
            .next()
            .map(|(_, n)| multinomial_variance(n as f64, self.sum_inv))
    }
}
