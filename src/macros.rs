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
