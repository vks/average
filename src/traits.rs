/// Estimate a statistic of a sequence of numbers ("population").
pub trait Estimate {
    /// Add an observation sampled from the population.
    fn add(&mut self, x: f64);

    /// Estimate the statistic of the population.
    fn estimate(&self) -> f64;
}

/// Merge another sample into this one.
pub trait Merge {
    fn merge(&mut self, other: &Self);
}

/// Get the bins and ranges from a histogram.
pub trait Histogram:
    where for<'a> &'a Self: IntoIterator<Item = ((f64, f64), u64)>
{
    /// Return the bins of the histogram.
    fn bins(&self) -> &[u64];

    /// Return an iterator over the bins normalized by the bin widths.
    #[inline]
    fn normalized_bins(&self) -> IterNormalized<<&Self as IntoIterator>::IntoIter> {
        IterNormalized { histogram_iter: self.into_iter() }
    }

    /// Return an iterator over the bin widths.
    #[inline]
    fn widths(&self) -> IterWidths<<&Self as IntoIterator>::IntoIter> {
        IterWidths { histogram_iter: self.into_iter() }
    }

    /// Return an iterator over the bin centers.
    #[inline]
    fn centers(&self) -> IterBinCenters<<&Self as IntoIterator>::IntoIter> {
        IterBinCenters { histogram_iter: self.into_iter() }
    }
}

/// Iterate over the bins normalized by bin width.
pub struct IterNormalized<T>
    where T: Iterator<Item = ((f64, f64), u64)>
{
    histogram_iter: T,
}

impl<T> Iterator for IterNormalized<T>
    where T: Iterator<Item = ((f64, f64), u64)>
{
    type Item = f64;

    #[inline]
    fn next(&mut self) -> Option<f64> {
        self.histogram_iter.next().map(|((a, b), count)| (count as f64) / (b - a))
    }
}

/// Iterate over the widths of the bins.
pub struct IterWidths<T>
    where T: Iterator<Item = ((f64, f64), u64)>
{
    histogram_iter: T,
}

impl<T> Iterator for IterWidths<T>
    where T: Iterator<Item = ((f64, f64), u64)>
{
    type Item = f64;

    #[inline]
    fn next(&mut self) -> Option<f64> {
        self.histogram_iter.next().map(|((a, b), _)| b - a)
    }
}

/// Iterate over the bin centers.
pub struct IterBinCenters<T>
    where T: Iterator<Item = ((f64, f64), u64)>
{
    histogram_iter: T,
}

impl<T> Iterator for IterBinCenters<T>
    where T: Iterator<Item = ((f64, f64), u64)>
{
    type Item = f64;

    #[inline]
    fn next(&mut self) -> Option<f64> {
        self.histogram_iter.next().map(|((a, b), _)| 0.5 * (a + b))
    }
}
