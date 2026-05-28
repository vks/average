use average::{ExpMovingMean, Merge};

fn main() {
    let mut a = ExpMovingMean::with_alpha(0.5);
    let b = ExpMovingMean::with_alpha(0.5);
    a.merge(&b); // should not compile: Merge is not implemented for ExpMovingMean
}
