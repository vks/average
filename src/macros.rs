/// Assert that two numbers are almost equal to each other.
///
/// On panic, this macro will print the values of the expressions with their
/// debug representations.
#[macro_export]
macro_rules! assert_almost_eq {
    ($a:expr, $b:expr, $prec:expr) => (
        let diff = ($a - $b).abs();
        if diff > $prec {
            panic!(format!(
                "assertion failed: `abs(left - right) = {:.1e} < {:e}`, \
                 (left: `{}`, right: `{}`)",
                diff, $prec, $a, $b));
        }
    );
}

/// Concatenate several iterative estimators into one.
///
/// `$name` is the name of the new struct. `$statistic` is the name of a
/// statistic and must exist as a method of the corresponding type `$estimator`.
/// `$estimator` must have an `add` method for adding new observations to the
/// sample (taking an `f64` as an argument). It must also implement `Default`.
///
/// If the short syntax is used, the fields will be named `$statistic`. Use the
/// long syntax and `$field` to give them explicit names. The long syntax also
/// supports calculating several statistics from one estimator.
///
/// For moments, only an estimator for the highest moment should be used and
/// reused for the lower moments (see the example below).
///
/// The following methods will be implemented: `new`, `add`, `$statistic`.
///
/// The following traits will be implemented: `Default`, `FromIterator<f64>`.
///
///
/// # Examples
///
/// ```
/// # extern crate core;
/// # #[macro_use] extern crate average;
/// # fn main() {
/// use average::{Min, Max, Estimate};
///
/// concatenate!(MinMax, [Min, min], [Max, max]);
///
/// let s: MinMax = (1..6).map(Into::into).collect();
///
/// assert_eq!(s.min(), 1.0);
/// assert_eq!(s.max(), 5.0);
/// # }
/// ```
///
/// The generated code looks roughly like this:
///
/// ```
/// # use average::{Min, Max, Estimate};
/// #
/// struct MinMax {
///     min: Min,
///     max: Max,
/// }
///
/// impl MinMax {
///     pub fn new() -> MinMax {
///         MinMax { min: Min::default(), max: Max::default() }
///     }
///
///     pub fn add(&mut self, x: f64) {
///         self.min.add(x);
///         self.max.add(x);
///     }
///
///     pub fn min(&self) -> f64 {
///         self.min.min()
///     }
///
///     pub fn max(&self) -> f64 {
///         self.max.max()
///     }
/// }
/// ```
///
/// If you want to calculate the mean, variance and the median in one pass, you
/// can do the following:
///
/// ```
/// # extern crate core;
/// # #[macro_use] extern crate average;
/// # fn main() {
/// use average::{Variance, Quantile, Estimate};
///
/// concatenate!(Estimator,
///     [Variance, variance, mean, sample_variance],
///     [Quantile, quantile, quantile]);
/// # }
/// ```
#[macro_export]
macro_rules! concatenate {
    ( $name:ident, $([$estimator:ident, $statistic:ident]),+ ) => {
        concatenate!( $name, $([$estimator, $statistic, $statistic]),* )
    };
    ( $name:ident, $( [$estimator:ident, $field:ident, $($statistic:ident),+] ),+ ) => {
        struct $name {
        $(
            $field: $estimator,
        )*
        }

        impl $name {
            #[inline]
            pub fn new() -> $name {
                $name {
                $(
                    $field: ::core::default::Default::default(),
                )*
                }
            }

            #[inline]
            pub fn add(&mut self, x: f64) {
                $(
                    self.$field.add(x);
                )*
            }

            $( $(
                #[inline]
                pub fn $statistic(&self) -> f64 {
                    self.$field.$statistic()
                }
            )* )*
        }

        impl Default for $name {
            fn default() -> $name {
                $name::new()
            }
        }

        impl_from_iterator!($name);
    };
}

/// Implement `FromIterator<f64>` for an iterative estimator.
#[macro_export]
macro_rules! impl_from_iterator {
    ( $name:ident ) => {
        impl ::core::iter::FromIterator<f64> for $name {
            fn from_iter<T>(iter: T) -> $name
                where T: IntoIterator<Item=f64>
            {
                let mut e = $name::new();
                for i in iter {
                    e.add(i);
                }
                e
            }
        }
    };
}
