/// Estimate a statistic of a sequence of numbers ("population").
pub trait Estimate {
    /// Add an observation sampled from the population.
    fn add(&mut self, x: f64);

    /// Estimate the statistic of the population.
    fn estimate(&self) -> f64;
}

/// Merge with another estimator.
pub trait Merge {
    /// Merge the other estimator into this one.
    ///
    /// Both estimators are assumed to be fed samples from the same population.
    ///
    /// This method is useful for parallelizing the calculation of estimates:
    /// ```
    /// use average::{Estimate, Mean, Merge};
    ///
    /// let data = &[1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    ///
    /// let mut thread1 = std::thread::spawn(move || -> Mean {
    ///     let mut avg = Mean::new();
    ///     for &x in &data[..5] {
    ///         avg.add(x);
    ///     }
    ///     avg
    /// });
    /// let mut thread2 = std::thread::spawn(move || -> Mean {
    ///     let mut avg = Mean::new();
    ///     for &x in &data[5..] {
    ///         avg.add(x);
    ///     }
    ///     avg
    /// });
    ///
    /// let mut avg = thread1.join().unwrap();
    /// avg.merge(&thread2.join().unwrap());
    /// assert_eq!(avg.mean(), 5.5);
    /// ```
    fn merge(&mut self, other: &Self);
}

/// Calculate the multinomial variance. Relevant for histograms.
#[inline(always)]
fn multinomial_variance(n: f64, n_tot_inv: f64) -> f64 {
    n * (1. - n * n_tot_inv)
}

/// Get the bins and ranges from a histogram.
pub trait Histogram
where
    for<'a> &'a Self: IntoIterator<Item = ((f64, f64), u64)>,
{
    /// Return the bins of the histogram.
    fn bins(&self) -> &[u64];

    /// Estimate the variance for the given bin.
    ///
    /// The square root of this estimates the error of the bin count.
    #[inline]
    fn variance(&self, bin: usize) -> f64 {
        let count = self.bins()[bin];
        let sum: u64 = self.bins().iter().sum();
        multinomial_variance(count as f64, 1. / (sum as f64))
    }

    /// Return an iterator over the bins normalized by the bin widths.
    #[inline]
    fn normalized_bins(&self) -> IterNormalized<<&Self as IntoIterator>::IntoIter> {
        IterNormalized {
            histogram_iter: self.into_iter(),
        }
    }

    /// Return an iterator over the bin widths.
    #[inline]
    fn widths(&self) -> IterWidths<<&Self as IntoIterator>::IntoIter> {
        IterWidths {
            histogram_iter: self.into_iter(),
        }
    }

    /// Return an iterator over the bin centers.
    #[inline]
    fn centers(&self) -> IterBinCenters<<&Self as IntoIterator>::IntoIter> {
        IterBinCenters {
            histogram_iter: self.into_iter(),
        }
    }

    /// Return an iterator over the bin variances.
    ///
    /// This is more efficient than calling `variance()` for each bin.
    #[inline]
    fn variances(&self) -> IterVariances<<&Self as IntoIterator>::IntoIter> {
        let sum: u64 = self.bins().iter().sum();
        IterVariances {
            histogram_iter: self.into_iter(),
            sum_inv: 1. / (sum as f64),
        }
    }
}

/// Iterate over the bins normalized by bin width.
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
