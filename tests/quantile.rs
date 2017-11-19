extern crate average;

use average::{Estimate, Quantile};

#[test]
fn few_observations() {
    let mut q = Quantile::new(0.5);
    assert_eq!(q.len(), 0);
    assert_eq!(q.quantile(), 0.);
    q.add(1.);
    assert_eq!(q.len(), 1);
    assert_eq!(q.quantile(), 1.);
    q.add(2.);
    assert_eq!(q.len(), 2);
    assert_eq!(q.quantile(), 1.5);
    q.add(3.);
    assert_eq!(q.len(), 3);
    assert_eq!(q.quantile(), 2.);
    q.add(4.);
    assert_eq!(q.len(), 4);
    assert_eq!(q.quantile(), 2.5);
}

#[test]
fn percentile_99_9() {
    let observations = [
        20., 20., 27., 28., 28., 34., 32., 32., 34., 33., 32., 30., 32., 30., 35., 27., 33., 26., 33.,
        26., 35., 32., 37., 30., 38., 26., 39., 37., 42., 29., 45., 27., 51., 33., 46., 38., 48., 31.,
        42., 35., 40., 29., 43., 30., 40., 29., 35., 34., 40., 24., 48., 32., 51., 33., 32., 32., 30.,
        28., 29., 31., 29., 29., 27., 2., 35., 3., 34., 28., 1., 25., 29., 28., 30., 27., 28., 3., 16.,
        31., 18., 28., 22., 30., 16., 27., 21., 6., 20., 28., 20., 6., 13., 20., 20., 30., 30., 28.,
        29., 29., 28., 27., 35., 25., 36., 25., 33., 27., 38., 34., 36., 33., 33., 41., 35., 34., 34.,
        37., 5., 31., 39., 34., 35., 41., 29., 39., 30., 37., 33., 33., 36., 33., 42., 32., 45., 33.,
        48., 31., 44., 30., 43., 29., 33., 29., 34., 33., 35., 33., 43., 24., 36., 29., 34., 26., 35.,
        6., 43., 28., 35., 28., 36., 4., 40., 25., 45., 20., 20., 30., 21., 28., 25., 36., 30., 37.,
        29., 40., 27., 40., 29., 37., 30., 40., 32., 45., 30., 42., 35., 44., 35., 38., 36., 37., 32.,
        35., 37., 39., 35., 36., 32., 35., 34., 36., 32., 37., 30., 27., 27., 42., 26., 39., 24., 42.,
        28., 43., 24., 44., 27., 40., 31., 29., 21., 28., 21., 38., 22., 35., 27., 25., 21., 30., 1.,
        30., 17., 26., 18., 28., 18., 2., 18., 30., 17., 30., 16., 28., 16., 28., 22., 26., 21., 23.,
        20., 20., 30., 21., 38., 23., 37., 28., 34., 27., 33., 29., 34., 26., 39., 26., 44., 25., 44.,
        26., 50., 25., 55., 24., 56., 31., 48., 35., 49., 36., 45., 40., 49., 30., 48., 29., 1., 35.,
        47., 35., 5., 27., 47., 22., 41., 26., 43., 26., 51., 1., 46., 1., 40., 1., 40., 0., 20., 20.,
        30., 22., 28., 25., 34., 27., 36., 29., 33., 35., 34., 32., 35., 29., 35., 3., 29., 31., 22.,
        38., 23., 4., 28., 32., 31., 31., 29., 4., 28., 2., 33., 3., 27., 4., 32., 29., 34., 27., 32.,
        2., 32., 26., 38., 1., 37., 2., 39., 38., 35., 41., 32., 0., 20., 20., 21., 28., 26., 30., 26.,
        36., 30., 37., 32., 40., 37., 41., 40., 36., 40., 38., 42., 42., 42., 46., 39., 43., 38., 48.,
        36., 51., 36., 48., 41., 46., 42., 46., 40., 40., 45., 37., 37., 40., 40., 36., 44., 42., 37.,
        37., 37., 39., 40., 37., 37., 28., 31., 28., 27., 33., 32., 4., 26., 34., 20., 20., 31., 28.,
        29., 47., 32., 31., 30., 32., 37., 33., 44., 35., 48., 33., 43., 28., 36., 30., 40., 33., 41.,
        33., 20., 20., 30., 30., 33., 28., 36., 36., 30., 7., 31., 5., 29., 35., 33., 33., 35., 34.,
        10., 38., 46., 44., 40., 39., 38., 43., 32., 39., 41., 40., 41., 39., 44., 44., 41., 43., 30.,
        27., 40., 26., 37., 27., 36., 29., 35., 27., 40., 28., 39., 28., 38., 29., 28., 23., 33., 23.,
        33., 23., 35., 25., 36., 24., 34., 20., 32., 18., 29., 21., 6., 15., 20., 8., 20., 5., 6., 6.,
        17., 4., 2., 7., 5., 7., 17., 3., 19., 6., 19., 5., 12., 4., 13., 2., 29., 3., 31., 0., 20.,
        20., 22., 19., 30., 26., 24., 26., 23., 26., 20., 27., 35., 3., 34., 33., 35., 29., 43., 28.,
        53., 29., 51., 0., 20., 20., 31., 28., 29., 47., 32., 39., 33., 39., 36., 42., 33., 43., 34.,
        45., 32., 48., 32., 47., 33., 49., 29., 50., 30., 48., 33., 45., 40., 43., 36., 43., 36., 42.,
        36., 47., 40., 46., 40., 46., 39., 41., 38., 37., 47., 4., 49., 40., 39., 3., 41., 1., 37., 3.,
        41., 6., 41., 36., 36., 40., 41., 39., 40., 45., 34., 30., 25., 31., 27., 29., 23., 26., 18.,
        25., 18., 20., 20., 30., 20., 31., 29., 29., 33., 37., 34., 37., 37., 33., 35., 37., 39., 35.,
        31., 29., 33., 34., 32., 37., 31., 36., 37., 35., 42., 36., 40., 29., 29., 27., 30., 29., 30.,
        4., 33., 24., 37., 28., 32., 25., 30., 27., 5., 29., 24., 21., 27., 25., 5., 20., 20., 11.,
        22., 15., 21., 22., 19., 20., 17., 20., 23., 24., 5., 26., 14., 24., 16., 20., 18., 4., 18.,
        14., 4., 20., 20., 20., 28., 24., 29., 30., 29., 35., 27., 36., 32., 34., 34., 31., 33., 31.,
        38., 31., 36., 30., 38., 31., 37., 36., 31., 36., 36., 38., 38., 40., 36., 43., 28., 49., 35.,
        35., 37., 40., 38., 5., 41., 38., 41., 39., 39., 40., 31., 38., 29., 39., 31., 37., 33., 37.,
        32., 33., 34., 3., 28., 27., 29., 30., 32., 29., 26., 28., 25., 27., 21., 33., 19., 26., 16.,
        27., 15., 26., 16., 24., 17., 27., 18., 22., 11., 18., 4., 22., 12., 19., 10., 22., 8., 24.,
        8., 20., 9., 18., 7., 23., 9., 19., 8., 21., 8., 21., 8., 22., 5., 22., 5., 25., 5., 24., 5.,
        26., 5., 20., 20., 29., 29., 31., 27., 33., 29., 31., 28., 42., 36., 38., 37., 30., 43., 31.,
        41., 29., 34., 25., 35., 30., 37., 32., 39., 32., 36., 31., 39., 33., 36., 36., 31., 43., 34.,
        41., 2., 45., 2., 38., 30., 45., 37., 42., 39., 39., 41., 40., 39., 39., 41., 39., 36., 41.,
        40., 40., 39., 40., 37., 47., 1., 30., 26., 29., 27., 35., 3., 32., 22., 26., 22., 29., 20.,
        28., 2., 33., 2., 33., 21., 36., 2., 34., 20., 31., 21., 29., 23., 6., 22., 24., 20., 23., 20.,
        22., 15., 27., 14., 20., 20., 27., 27., 26., 33., 25., 31., 29., 32., 32., 30., 34., 32., 39.,
        37., 34., 38., 35., 35., 40., 3., 33., 40., 39., 3., 28., 41., 34., 39., 28., 30., 28., 26.,
        18., 34., 17., 33., 22., 39., 19., 37., 30., 28., 33., 33., 31., 38., 29., 33., 29., 31., 37.,
        22., 38., 21., 3., 28., 38., 33., 30., 28., 32., 25., 27., 23., 23., 23., 20., 23., 22., 20.,
        24., 22., 22., 22., 22., 20., 18., 25., 21., 26., 21., 24., 20., 22., 12., 29., 12., 29., 16.,
        23., 10., 19., 13., 8., 18., 20., 19., 22., 3., 30., 4., 30., 22., 5., 25., 29., 6., 27., 8.,
        18., 8., 17., 6., 16., 7., 16., 3., 14., 3., 9., 8., 20., 20., 27., 28., 30., 30., 33., 35.,
        38., 37., 41., 1., 28., 29., 4., 27., 29., 24., 31., 30., 35., 30., 43., 38., 50., 8., 51., 4.,
        55., 40., 7., 45., 51., 42., 52., 44., 5., 38., 41., 40., 45., 42., 37., 34., 7., 34., 42., 2.,
        34., 26., 36., 17., 29., 16., 30., 15., 26., 7., 27., 7., 27., 20., 20., 29., 29., 33., 38.,
        37., 41., 40., 40., 43., 36., 39., 28., 46., 28., 38., 29., 39., 36., 40., 31., 38., 32., 37.,
        30., 36., 32., 37., 34., 36., 39., 35., 34., 33., 33., 25., 30., 5., 25., 6., 25., 24., 23.,
        28., 20., 31., 6., 23., 24., 22., 22., 22., 23., 26., 21., 24., 21., 27., 17., 28., 20., 28.,
        21., 26., 26., 22., 25., 22., 21., 4., 17., 21., 6., 20., 18., 12., 3., 13., 10., 14., 12.,
        14., 3., 12., 7., 14., 11., 17., 15., 16., 11., 15., 6., 15., 6., 31., 4., 20., 20., 30., 21.,
        33., 30., 31., 29., 26., 31., 26., 34., 33., 40., 32., 44., 32., 47., 29., 44., 28., 46., 28.,
        51., 35., 51., 34., 45., 31., 48., 37., 43., 32., 41., 32., 34., 33., 31., 31., 33., 37., 38.,
        33., 25., 29., 34., 25., 24., 21., 31., 21., 29., 24., 24., 17., 20., 17., 16., 22., 16., 23.,
        19., 24., 15., 20., 17., 20., 20., 19., 16., 20., 19., 17., 18., 19., 20., 18., 22., 17., 23.,
        17., 20., 15., 6., 16., 4., 15., 16., 1., 16., 0., 20., 20., 29., 29., 27., 30., 32., 24., 25.,
        35., 33., 34., 30., 30., 24., 31., 26., 32., 27., 32., 26., 28., 26., 30., 38., 30., 36., 35.,
        38., 33., 34., 34., 37., 34., 35., 35., 37., 32., 34., 30., 5., 37., 32., 39., 29., 38., 33.,
        35., 35., 38., 30., 39., 27., 34., 31., 39., 30., 31., 21., 30., 22., 27., 24., 28., 26., 32.,
        29., 32., 33., 29., 34., 27., 34., 25., 34., 20., 42., 20., 41., 19., 42., 21., 44., 24., 43.,
        21., 42., 14., 40., 4., 41., 21., 40., 20., 42., 12., 36., 16., 34., 15., 20., 20., 30., 22.,
        28., 27., 37., 25., 41., 34., 40., 35., 40., 33., 7., 34., 41., 33., 48., 31., 47., 39., 51.,
        32., 47., 30., 44., 30., 34., 29., 43., 34., 41., 44., 41., 46., 32., 40., 34., 40., 34., 41.,
        32., 41., 32., 37., 35., 32., 29., 30., 26., 22., 26., 25., 15., 19., 15., 19., 14., 17., 16.,
        17., 17., 22., 1., 39., 3., 33., 20., 20., 28., 21., 38., 23., 40., 21., 39., 22., 34., 27.,
        35., 24., 38., 24., 39., 25., 36., 25., 34., 27., 33., 34., 33., 25., 24., 26., 26., 24., 25.,
        24., 1., 22., 36., 22., 37., 22., 38., 17., 40., 16., 33., 17., 33., 16., 37., 16., 35., 15.,
        36., 19., 29., 21., 30., 16., 30., 3., 39., 9., 3., 5., 35., 5., 38., 0., 20., 20., 20., 21.,
        21., 27., 26., 26., 36., 30., 37., 27., 37., 28., 37., 28., 41., 34., 38., 36., 44., 38., 44.,
        31., 38., 29., 39., 35., 41., 40., 40., 42., 40., 38., 35., 40., 40., 39., 32., 41., 34., 38.,
        30., 37., 34., 44., 33., 41., 36., 40., 38., 38., 40., 45., 39., 48., 38., 47., 36., 43., 34.,
        40., 39., 36., 35., 38., 38., 2., 39., 1., 23., 33., 1., 20., 20., 30., 22., 28., 30., 37.,
        28., 42., 31., 43., 30., 42., 30., 43., 29., 45., 31., 41., 32., 42., 29., 40., 33., 41., 30.,
        37., 32., 47., 35., 43., 34., 45., 32., 36., 38., 33., 37., 34., 39., 38., 35., 38., 1., 37.,
        38., 33., 36., 38., 40., 42., 2., 44., 4., 39., 3., 20., 20., 29., 29., 28., 38., 27., 48.,
        30., 42., 32., 43., 33., 36., 40., 42., 38., 46., 38., 38., 41., 38., 47., 3., 47., 41., 47.,
        44., 2., 44., 48., 42., 50., 42., 49., 42., 54., 52., 50., 51., 34., 49., 28., 55., 2., 48.,
        3., 42., 2., 37., 12., 44., 12., 38., 12., 37., 2., 37., 0., 20., 20., 30., 19., 38., 29., 36.,
        30., 37., 29., 33., 31., 33., 29., 38., 36., 36., 33., 30., 30., 38., 35., 30., 35., 33., 36.,
        36., 37., 40., 33., 37., 27., 37., 27., 37., 33., 44., 39., 45., 40., 44., 41., 49., 40., 48.,
        3., 43., 0., 20., 20., 27., 28., 30., 34., 5., 37., 30., 37., 26., 45., 6., 38., 36., 37., 34.,
        31., 34., 31., 37., 32., 33., 28., 40., 29., 42., 38., 38., 42., 37., 40., 39., 36., 40., 32.,
        43., 31., 41., 36., 36., 39., 37., 39., 4., 33., 39., 31., 38., 32., 45., 38., 29., 27., 30.,
        28., 30., 32., 28., 37., 28., 30., 27., 31., 28., 20., 20., 30., 27., 34., 28., 42., 28., 38.,
        31., 26., 32., 33., 32., 31., 31., 32., 26., 33., 35., 28., 43., 27., 39., 3., 45., 3., 44.,
        20., 20., 30., 19., 39., 22., 40., 20., 41., 26., 36., 35., 43., 33., 40., 29., 38., 28., 31.,
        28., 4., 23., 35., 28., 35., 34., 33., 42., 34., 40., 36., 42., 28., 43., 37., 42., 38., 40.,
        34., 39., 35., 36., 33., 39., 34., 33., 33., 34., 33., 34., 31., 29., 15., 30., 14., 25., 14.,
        21., 19., 22., 19., 17., 23., 1., 23., 14., 17., 13., 17., 10., 17., 8., 16., 9., 17., 12.,
        17., 12., 17., 11., 20., 14., 19., 5., 19., 20., 20., 30., 22., 28., 29., 32., 31., 33., 29.,
        34., 31., 31., 32., 34., 35., 33., 36., 32., 36., 32., 36., 31., 30., 30., 34., 34., 35., 44.,
        36., 47., 41., 42., 38., 38., 33., 37., 35., 37., 36., 2., 45., 39., 47., 38., 41., 43., 42.,
        41., 43., 33., 3., 32., 42., 32., 45., 28., 4., 39., 40., 31., 47., 20., 39., 14., 27., 17.,
        27., 18., 26., 17., 32., 3., 32., 17., 20., 20., 29., 29., 27., 31., 33., 36., 36., 32., 40.,
        32., 41., 37., 43., 38., 46., 33., 41., 31., 41., 36., 43., 32., 42., 37., 41., 34., 36., 55.,
        31., 49., 32., 43., 33., 42., 30., 43., 32., 42., 4., 47., 3., 46., 38., 2., 37., 44., 38.,
        43., 4., 41., 35., 41., 23., 25., 17., 25., 23., 29., 29., 25., 26., 28., 26., 26., 25., 26.,
        29., 5., 29., 28., 27., 32., 25., 33., 25., 35., 20., 34., 22., 36., 20., 23., 13., 25., 16.,
        22., 12., 4., 15., 19., 15., 21., 11., 19., 12., 20., 20., 29., 29., 33., 32., 33., 33., 34.,
        28., 41., 28., 37., 29., 45., 28., 41., 30., 37., 30., 31., 36., 30., 35., 36., 27., 36., 25.,
        38., 20., 20., 20., 30., 22., 28., 30., 36., 32., 8., 39., 34., 45., 35., 49., 37., 51., 32.,
        46., 39., 4., 51., 30., 33., 28., 32., 23., 41., 3., 37., 24., 34., 24., 33., 25., 27., 25.,
        31., 23., 30., 26., 27., 32., 28., 31., 29., 33., 29., 37., 29., 37., 25., 44., 26., 41., 20.,
        32., 17., 32., 22., 30., 23., 26., 24., 5., 23., 18., 29., 17., 5., 22., 6., 22., 27., 26.,
        24., 21., 28., 22., 30., 5., 31., 20., 22., 20., 32., 13., 27., 15., 29., 13., 5., 19., 24.,
        14., 28., 14., 28., 2., 28., 16., 6., 16., 3., 16., 6., 16., 18., 14., 2., 21., 17., 21., 24.,
        20., 18., 18., 4., 18., 7., 18., 20., 20., 28., 22., 32., 33., 33., 42., 34., 40., 35., 35.,
        29., 3., 30., 36., 26., 35., 30., 39., 28., 36., 24., 4., 20., 20., 21., 28., 26., 30., 26.,
        31., 37., 33., 38., 31., 38., 32., 38., 29., 44., 30., 42., 28., 39., 32., 40., 32., 45., 37.,
        41., 28., 49., 31., 47., 35., 45., 33., 49., 32., 43., 34., 44., 32., 46., 40., 48., 39., 6.,
        31., 39., 25., 39., 23., 39., 20., 36., 22., 37., 18., 31., 19., 37., 1., 35., 20., 37., 27.,
        35., 34., 33., 33., 38., 30., 26., 39., 28., 32., 33., 30., 29., 36., 16., 27., 20., 20., 29.,
        29., 27., 31., 28., 29., 34., 2., 26., 4., 31., 27., 34., 36., 34., 49., 40., 33., 26., 31.,
        29., 32., 20., 35., 22., 32., 25., 36., 30., 32., 36., 5., 36., 26., 31., 34., 3., 29., 28.,
        29., 22., 33., 24., 29., 25., 5., 22., 19., 26., 19., 24., 18., 27., 17., 4., 23., 24., 16.,
        31., 14., 29., 1., 30., 15., 5., 14., 6., 14., 20., 13., 4., 18., 23., 13., 22., 16., 9., 21.,
        13., 20., 5., 7., 12., 11., 12., 11., 12., 9., 10., 9., 10., 9., 10., 10., 9., 9., 8., 6., 10.,
        6., 26., 6., 27., 5., 30., 8., 29., 3., 31., 7., 20., 20., 27., 27., 33., 30., 37., 26., 20.,
        20., 29., 29., 32., 31., 33., 29., 32., 32., 31., 32., 31., 32., 31., 33., 27., 32., 29., 32.,
        28., 31., 29., 31., 32., 32., 31., 32., 33., 30., 33., 33., 36., 33., 36., 33., 35., 34., 31.,
        36., 35., 34., 3., 30., 27., 33., 28., 41., 26., 41., 26., 44., 27., 43., 30., 40., 28., 37.,
        30., 33., 30., 34., 33., 36., 34., 1., 35., 3., 36., 20., 20., 30., 22., 28., 24., 37., 28.,
        39., 26., 37., 33., 40., 36., 43., 38., 44., 35., 41., 32., 45., 29., 46., 28., 49., 29., 46.,
        32., 52., 33., 45., 35., 42., 35., 42., 35., 37., 36., 41., 32., 38., 30., 43., 29., 39., 30.,
        43., 5., 42., 33., 42., 7., 46., 4., 45., 27., 40., 1., 20., 20., 21., 30., 25., 33., 27., 34.,
        29., 33., 35., 33., 33., 33., 33., 32., 41., 33., 38., 33., 42., 31., 44., 33., 35., 40., 35.,
        33., 39., 35., 39., 35., 6., 35., 33., 39., 36., 34., 50., 30., 53., 38., 38., 39., 3., 33.,
        38., 33., 34., 32., 33., 24., 33., 20., 20., 30., 21., 38., 25., 39., 32., 43., 33., 40., 32.,
        42., 29., 39., 29., 42., 30., 38., 30., 38., 28., 37., 31., 40., 37., 36., 32., 38., 35., 42.,
        35., 40., 33., 42., 29., 45., 31., 38., 3., 43., 2., 46., 35., 41., 34., 45., 40., 48., 3.,
        51., 2., 43., 2., 54., 25., 58., 2., 53., 1., 51., 0., 20., 20., 21., 27., 21., 37., 30., 37.,
        34., 37., 31., 40., 32., 42., 33., 46., 30., 38., 28., 35., 26., 31., 26., 36., 27., 34., 30.,
        35., 25., 31., 23., 34., 1., 35., 26., 32., 38., 39., 33., 38., 40., 32., 2., 34., 1., 32., 0.,
        20., 20., 28., 22., 30., 23., 33., 26., 40., 34., 37., 36., 35., 30., 37., 28., 40., 29., 39.,
        28., 43., 28., 41., 27., 41., 29., 38., 34., 40., 38., 33., 40., 33., 46., 35., 44., 35., 43.,
        33., 41., 32., 37., 34., 43., 32., 41., 34., 43., 36., 41., 32., 45., 30., 46., 37., 6., 41.,
        2., 43., 5., 23., 32., 3., 28., 23., 25., 1., 19., 13., 15., 24., 15., 20., 20., 28., 27., 31.,
        33., 33., 1., 25., 32., 4., 35., 24., 35., 26., 38., 29., 3., 28., 38., 1., 40., 25., 42., 26.,
        34., 31., 40., 28., 41., 20., 20., 30., 30., 38., 32., 37., 5., 40., 29., 41., 31., 35., 28.,
        32., 23., 29., 28., 28., 30., 30., 35., 41., 33., 41., 33., 40., 28., 45., 32., 42., 33., 38.,
        34., 39., 26., 38., 32., 38., 36., 32., 36., 31., 39., 32., 32., 32., 33., 32., 34., 24., 2.,
        20., 33., 2., 28., 5., 35., 3., 29., 5., 33., 3., 35., 2., 41., 20., 20., 29., 29., 39., 31.,
        44., 39., 37., 37., 44., 41., 45., 34., 46., 40., 43., 46., 38., 44., 42., 42., 40., 43., 43.,
        46., 32., 20., 20., 31., 28., 29., 47., 5., 43., 20., 45., 27., 41., 34., 38., 41., 34., 42.,
        36., 42., 24., 35., 25., 36., 27., 34., 31., 33., 33., 34., 36., 38., 34., 37., 33., 31., 28.,
        32., 33., 32., 37., 30., 40., 34., 38., 2., 31., 38., 0., 20., 20., 29., 29., 31., 25., 42.,
        26., 45., 30., 45., 31., 45., 31., 38., 32., 40., 27., 41., 27., 43., 26., 40., 22., 39., 28.,
        36., 37., 34., 38., 32., 30., 32., 30., 31., 31., 30., 33., 31., 33., 34., 39., 32., 1., 31.,
        32., 30., 29., 24., 25., 22., 34., 4., 34., 28., 29., 4., 35., 30., 25., 21., 19., 21., 17.,
        19., 16., 17., 15., 14., 19., 15., 18., 14., 21., 5., 27., 10., 25., 11., 24., 16., 1., 13.,
        35., 4., 42., 0., 20., 20., 21., 27., 27., 28., 37., 25., 31., 27., 31., 29., 38., 33., 38.,
        32., 36., 34., 33., 33., 33., 36., 30., 32., 34., 36., 33., 40., 38., 43., 35., 36., 40., 31.,
        35., 47., 35., 44., 34., 42., 32., 40., 30., 38., 32., 36., 33., 39., 18., 34., 21., 34., 3.,
        34., 21., 33., 20., 20., 30., 22., 28., 29., 30., 33., 38., 31., 43., 34., 48., 31., 44., 31.,
        44., 34., 45., 37., 52., 36., 49., 34., 46., 38., 48., 39., 43., 43., 40., 27., 31., 35., 33.,
        36., 22., 34., 31., 33., 34., 24., 30., 25., 34., 23., 23., 23., 24., 32., 21., 32., 18., 32.,
        24., 31., 23., 31., 3., 20., 20., 30., 21., 38., 23., 39., 25., 32., 26., 30., 29., 28., 31.,
        34., 29., 36., 36., 32., 35., 34., 33., 32., 36., 40., 38., 33., 42., 37., 38., 42., 37., 45.,
        38., 42., 39., 43., 41., 36., 45., 26., 20., 20., 31., 28., 29., 47., 32., 39., 33., 42., 39.,
        42., 35., 43., 37., 33., 39., 30., 37., 36., 38., 35., 37., 37., 38., 46., 36., 41., 36., 50.,
        33., 42., 41., 20., 20., 22., 29., 24., 32., 24., 25., 32., 35., 33., 46., 26., 27., 27., 30.,
        21., 23., 21., 29., 22., 28., 26., 26., 24., 25., 24., 23., 23., 22., 26., 27., 28., 24., 32.,
        26., 32., 26., 26., 27., 25., 30., 25., 7., 28., 30., 31., 31., 31., 31., 29., 32., 29., 37.,
        5., 38., 2., 37., 2., 37., 35., 37., 34., 35., 38., 38., 34., 2., 42., 29., 2., 27., 42., 21.,
        45., 3., 39., 4., 42., 17., 43., 2., 41., 15., 7., 13., 4., 20., 20., 29., 30., 38., 31., 37.,
        30., 37., 31., 38., 28., 32., 31., 31., 32., 31., 25., 32., 27., 29., 25., 28., 28., 33., 25.,
        34., 27., 29., 33., 28., 27., 26., 32., 31., 33., 33., 36., 32., 36., 34., 35., 35., 33., 32.,
        38., 30., 41., 22., 41., 4., 39., 23., 28., 17., 32., 3., 37., 18., 36., 21., 42., 5., 43.,
        22., 41., 17., 24., 4., 26., 14., 23., 17., 20., 17., 21., 20., 24., 24., 23., 23., 6., 24.,
        16., 20., 20., 20., 33., 5., 31., 1., 20., 20., 29., 30., 38., 30., 37., 28., 43., 34., 38.,
        37., 35., 30., 43., 26., 45., 27., 49., 30., 48., 24., 41., 21., 43., 21., 43., 19., 41., 20.,
        39., 18., 42., 29., 36., 32., 37., 32., 41., 28., 47., 23., 45., 22., 48., 21., 43., 22., 36.,
        29., 36., 25., 33., 1., 26., 1., 31., 21., 1., 19., 27., 21., 26., 28., 27., 28., 27., 30., 4.,
        26., 4., 28., 22., 26., 3., 28., 5., 27., 5., 29., 5., 23., 2., 24., 4., 26., 2., 24., 4., 26.,
        22., 24., 18., 3., 27., 25., 4., 27., 7., 24., 7., 24., 6., 27., 4., 26., 27., 24., 3., 23.,
        4., 20., 3., 16., 4., 24., 3., 16., 4., 24., 3., 16., 20., 20., 27., 28., 30., 34., 33., 32.,
        38., 37., 33., 32., 33., 27., 33., 24., 37., 25., 44., 27., 45., 26., 45., 27., 42., 26., 41.,
        32., 38., 32., 41., 32., 41., 42., 50., 20., 20., 29., 30., 29., 30., 36., 29., 43., 30., 54.,
        34., 54., 33., 48., 31., 48., 39., 48., 45., 48., 3., 55., 20., 20., 28., 30., 30., 34., 24.,
        32., 31., 33., 27., 30., 37., 29., 38., 31., 34., 33., 41., 36., 41., 39., 1., 30., 34., 30.,
        26., 34., 30., 33., 5., 33., 29., 34., 33., 39., 2., 33., 8., 39., 25., 39., 3., 32., 13., 21.,
        17., 21., 17., 22., 17., 25., 17., 20., 9., 23., 10., 23., 11., 24., 8., 27., 2., 26., 7., 24.,
        10., 24., 8., 24., 7., 26., 7., 23., 5., 24., 6., 26., 6., 27., 3., 24., 6., 25., 8., 25., 5.,
        7., 4., 21., 7., 26., 8., 27., 7., 31., 2., 44., 5., 39., 7., 41., 4., 41., 3., 39., 0., 20.,
        20., 28., 30., 38., 32., 36., 34., 37., 24., 36., 38., 35., 37., 34., 32., 30., 34., 33., 31.,
        35., 29., 34., 36., 27., 33., 27., 37., 29., 45., 35., 42., 33., 40., 33., 39., 37., 37., 36.,
        34., 37., 33., 34., 38., 37., 33., 34., 38., 40., 34., 37., 38., 32., 37., 27., 28., 3., 33.,
        30., 3., 25., 3., 27., 6., 25., 25., 2., 27., 2., 24., 2., 26., 27., 2., 30., 4., 32., 3., 30.,
        2., 30., 3., 22., 5., 27., 33., 18., 29., 20., 28., 2., 21., 3., 27., 19., 28., 21., 39., 1.,
        29., 0., 20., 20., 30., 30., 38., 31., 39., 35., 40., 33., 40., 34., 40., 38., 43., 37., 38.,
        33., 34., 30., 43., 28., 47., 29., 42., 28., 40., 30., 40., 26., 37., 33., 36., 32., 2., 26.,
        36., 31., 40., 36., 38., 33., 36., 31., 35., 31., 42., 30., 40., 26., 35., 27., 35., 27., 39.,
        26., 36., 26., 33., 26., 29., 28., 29., 33., 28., 32., 29., 32., 30., 31., 29., 32., 27., 38.,
        25., 38., 27., 38., 30., 39., 28., 37., 36., 41., 35., 37., 35., 30., 34., 34., 27., 24., 27.,
        22., 24., 20., 22., 24., 26., 8., 29., 21., 27., 20., 20., 30., 30., 38., 31., 36., 35., 34.,
        34., 35., 31., 32., 34., 33., 36., 34., 36., 37., 28., 36., 30., 33., 26., 32., 25., 32., 26.,
        28., 28., 34., 25., 34., 29., 37., 23., 49., 1., 48., 1., 45., 4., 46., 3., 38., 20., 39., 23.,
        39., 3., 42., 29., 41., 2., 41., 28., 43., 32., 36., 5., 35., 35., 31., 6., 32., 4., 29., 43.,
        26., 45., 4., 50., 3., 43., 27., 40., 28., 4., 28., 38., 25., 34., 27., 33., 28., 3., 21., 27.,
        28., 4., 28., 25., 25., 23., 28., 27., 29., 18., 24., 26., 27., 5., 21., 24., 24., 27., 21.,
        26., 19., 3., 17., 6., 19., 19., 1., 30., 2., 36., 1., 31., 1., 29., 2., 20., 20., 21., 30.,
        27., 33., 28., 32., 32., 33., 32., 34., 30., 36., 32., 35., 35., 37., 29., 40., 6., 46., 32.,
        51., 7., 53., 45., 47., 43., 44., 39., 0., 20., 20., 29., 29., 27., 27., 29., 29., 37., 29.,
        36., 36., 30., 47., 28., 40., 36., 43., 37., 40., 36., 36., 35., 37., 36., 34., 35., 41., 42.,
        38., 39., 34., 44., 34., 38., 45., 43., 35., 44., 42., 44., 42., 43., 45., 36., 45., 34., 43.,
        35., 38., 38., 33., 42., 28., 27., 1., 29., 27., 28., 29., 20., 26., 13., 41., 18., 33., 3.,
        37., 17., 29., 2., 23., 14., 26., 14., 30., 15., 29., 14., 27., 10., 27., 14., 26., 12., 3.,
        12., 21., 2., 24., 13., 26., 12., 23., 14., 19., 1., 9., 5., 8., 6., 9., 6., 9., 5., 7., 3.,
        8., 3., 9., 4., 9., 6., 10., 3., 28., 1., 27., 3., 29., 3., 27., 3., 30., 2., 41., 2., 44., 1.,
        39., 0., 20., 20., 30., 22., 28., 28., 33., 30., 36., 30., 43., 31., 37., 29., 41., 33., 44.,
        31., 46., 29., 44., 34., 39., 33., 36., 34., 40., 29., 38., 22., 30., 33., 29., 42., 29., 35.,
        28., 31., 33., 14., 16., 15., 23., 14., 28., 17., 26., 16., 27., 17., 31., 18., 30., 17., 29.,
        2., 33., 13., 32., 19., 32., 2., 30., 16., 28., 16., 26., 20., 23., 19., 22., 20., 14., 12.,
        15., 9., 17., 10., 20., 10., 14., 9., 9., 12., 10., 12., 9., 13., 8., 10., 9., 9., 9., 9., 12.,
        9., 29., 3., 34., 2., 24., 7., 23., 5., 21., 4., 21., 2., 28., 2., 28., 1., 28., 2., 28., 1.,
        26., 0., 20., 20., 27., 28., 30., 30., 33., 33., 31., 30., 37., 32., 37., 27., 39., 30., 40.,
        32., 41., 36., 40., 36., 44., 41., 44., 41., 34., 43., 40., 40., 40., 40., 42., 42., 52., 36.,
        48., 33., 45., 36., 47., 35., 43., 38., 35., 33., 41., 35., 41., 33., 3., 38., 39., 35., 38.,
        37., 40., 33., 20., 20., 28., 21., 34., 25., 33., 26., 37., 34., 35., 35., 34., 31., 35., 31.,
        34., 32., 34., 34., 42., 39., 33., 38., 33., 37., 32., 41., 29., 42., 30., 42., 24., 41., 26.,
        30., 23., 37., 19., 23., 27., 23., 23., 24., 25., 23., 26., 22., 28., 23., 27., 18., 22., 19.,
        19., 18., 22., 18., 22., 22., 21., 17., 20., 20., 17., 19., 21., 18., 21., 26., 20., 21., 19.,
        15., 18., 14., 18., 4., 16., 12., 16., 20., 20., 30., 22., 30., 26., 30., 33., 36., 4., 29.,
        33., 27., 31., 26., 32., 30., 37., 32., 36., 33., 42., 30., 42., 32., 36., 36., 38., 48., 36.,
        47., 37., 44., 39., 43., 37., 37., 38., 35., 43., 41., 41., 37., 34., 31., 23., 19., 26., 19.,
        26., 16., 29., 23., 30., 2., 28., 18., 28., 17., 23., 3., 20., 12., 11., 7., 10., 7., 11., 7.,
        12., 7., 11., 7., 9., 7., 8., 9., 7., 7., 8., 6., 7., 7., 11., 8., 10., 6., 25., 4., 23., 4.,
        20., 20., 30., 30., 38., 35., 36., 36., 38., 39., 33., 32., 45., 30., 45., 36., 40., 31., 46.,
        30., 54., 34., 52., 30., 51., 31., 45., 36., 41., 39., 43., 38., 41., 32., 36., 37., 40., 37.,
        33., 38., 36., 31., 35., 6., 30., 5., 21., 22., 25., 21., 23., 20., 21., 26., 21., 22., 23.,
        23., 24., 24., 20., 24., 21., 31., 20., 31., 20., 32., 24., 27., 23., 25., 20., 19., 22., 21.,
        6., 21., 5., 20., 19., 18., 5., 22., 16., 18., 14., 13., 4., 17., 20., 16., 14., 15., 13., 12.,
        4., 16., 4., 18., 17., 13., 14., 18., 3., 20., 9., 20., 13., 20., 12., 21., 12., 21., 11., 18.,
        1., 22., 16., 15., 2., 21., 3., 19., 7., 13., 24., 5., 26., 6., 25., 4., 26., 5., 27., 16., 7.,
        18., 4., 15., 2., 18., 4., 15., 4., 20., 25., 5., 27., 5., 26., 3., 30., 2., 29., 13., 7., 12.,
        5., 12., 4., 17., 23., 3., 25., 4., 6., 5., 22., 15., 4., 16., 19., 4., 21., 3., 21., 11., 6.,
        12., 25., 12., 26., 11., 22., 2., 26., 3., 30., 6., 20., 20., 30., 30., 33., 35., 31., 37.,
        26., 35., 33., 41., 35., 43., 36., 39., 35., 42., 39., 44., 41., 45., 36., 44., 33., 47., 33.,
        48., 33., 46., 32., 48., 30., 48., 20., 20., 27., 27., 28., 31., 32., 32., 32., 29., 28., 24.,
        31., 23., 32., 31., 35., 32., 42., 32., 43., 4., 49., 2., 49., 28., 52., 34., 47., 34., 38.,
        30., 39., 33., 46., 6., 38., 24., 32., 36., 35., 20., 20., 29., 29., 27., 24., 34., 27., 31.,
        28., 30., 24., 29., 24., 24., 25., 28., 29., 28., 28., 29., 31., 28., 39., 34., 42., 37., 47.,
        3., 51., 35., 46., 38., 42., 45., 3., 41., 0., 20., 20., 28., 30., 30., 28., 32., 32., 31.,
        34., 30., 35., 31., 41., 37., 40., 29., 47., 36., 47., 37., 45., 37., 37., 31., 33., 31., 31.,
        32., 35., 34., 4., 38., 25., 40., 6., 32., 30., 35., 5., 44., 30., 39., 37., 42., 39., 49.,
        37., 55., 31., 47., 38., 35., 38., 41., 37., 44., 25., 42., 23., 41., 34., 38., 31., 3., 31.,
        4., 33., 43., 3., 47., 1., 51., 1., 50., 0., 20., 20., 27., 27., 31., 29., 34., 31., 39., 6.,
        45., 27., 45., 29., 41., 35., 41., 35., 38., 33., 40., 35., 34., 32., 39., 42., 37., 40., 33.,
        40., 39., 34., 31., 33., 31., 31., 36., 38., 34., 33., 35., 41., 38., 42., 34., 35., 34., 27.,
        22., 24., 26., 23., 32., 25., 28., 24., 26., 28., 31., 27., 29., 25., 26., 29., 25., 3., 12.,
        20., 20., 30., 23., 38., 23., 34., 21., 41., 22., 41., 29., 43., 28., 42., 33., 44., 34., 43.,
        32., 39., 36., 43., 35., 40., 39., 38., 38., 47., 41., 48., 41., 47., 40., 47., 41., 44., 36.,
        37., 36., 35., 32., 30., 41., 34., 43., 38., 40., 36., 39., 35., 41., 36., 35., 4., 37., 25.,
        41., 6., 39., 17., 22., 17., 20., 14., 21., 11., 22., 15., 22., 13., 20., 20., 30., 22., 38.,
        33., 38., 37., 39., 38., 38., 45., 35., 41., 38., 37., 37., 38., 38., 39., 40., 35., 43., 34.,
        45., 29., 48., 28., 47., 26., 39., 33., 47., 27., 43., 29., 44., 31., 46., 28., 42., 34., 34.,
        38., 4., 35., 38., 34., 39., 36., 38., 39., 39., 40., 37., 43., 32., 49., 28., 53., 3., 45.,
        31., 37., 30., 36., 4., 30., 3., 29., 3., 28., 0., 20., 20., 28., 21., 34., 25., 37., 26., 37.,
        27., 38., 30., 35., 26., 42., 29., 45., 27., 43., 31., 45., 32., 39., 29., 35., 33., 31., 33.,
        33., 37., 31., 38., 35., 34., 31., 37., 28., 34., 29., 28., 32., 28., 1., 20., 28., 19., 28.,
        17., 26., 19., 26., 21., 25., 16., 25., 22., 24., 24., 24., 27., 25., 6., 26., 27., 25., 25.,
        18., 32., 20., 33., 21., 27., 3., 24., 20., 24., 5., 26., 22., 18., 13., 19., 5., 22., 14.,
        17., 13., 19., 13., 17., 7., 18., 5., 17., 11., 3., 19., 19., 14., 19., 5., 20., 20., 21., 21.,
        27., 29., 28., 31., 33., 32., 32., 35., 38., 42., 39., 46., 41., 43., 39., 43., 31., 42., 31.,
        35., 31., 36., 31., 34., 37., 34., 35., 33., 37., 34., 37., 41., 38., 35., 36., 34., 38., 35.,
        40., 4., 43., 3., 39., 35., 37., 2., 27., 1., 30., 2., 20., 20., 28., 21., 38., 25., 36., 30.,
        37., 28., 42., 26., 42., 26., 37., 27., 34., 32., 35., 33., 35., 36., 39., 37., 34., 38., 36.,
        37., 39., 38., 43., 39., 32., 35., 40., 31., 34., 32., 30., 26., 31., 32., 32., 33., 37., 34.,
        33., 33., 34., 34., 28., 30., 3., 34., 28., 27., 2., 19., 5., 24., 17., 24., 18., 25., 20.,
        18., 21., 18., 22., 21., 17., 21., 15., 20., 15., 26., 25., 17., 25., 18., 14., 26., 16., 26.,
        6., 28., 3., 23., 17., 23., 24., 20., 19., 21., 19., 18., 19., 17., 14., 16., 11., 16., 10.,
        15., 13., 13., 13., 19., 13., 22., 11., 23., 10., 21., 15., 22., 14., 19., 11., 19., 7., 11.,
        5., 7., 5., 8., 5., 10., 5., 7., 5., 10., 5., 10., 3., 10., 3., 8., 1., 10., 0., 20., 20., 22.,
        21., 30., 26., 32., 36., 29., 34., 31., 35., 28., 33., 29., 31., 30., 30., 26., 29., 27., 31.,
        26., 31., 27., 26., 27., 27., 31., 29., 31., 35., 37., 38., 20., 20., 29., 29., 27., 27., 33.,
        29., 36., 36., 39., 3., 33., 7., 34., 31., 44., 4., 42., 30., 40., 39., 35., 37., 39., 3., 32.,
        3., 20., 20., 22., 27., 28., 29., 36., 31., 39., 30., 36., 5., 42., 34., 39., 33., 35., 30.,
        37., 31., 38., 28., 43., 33., 36., 32., 39., 36., 48., 41., 50., 39., 45., 5., 46., 2., 40.,
        35., 44., 34., 44., 3., 42., 4., 44., 33., 43., 2., 36., 36., 44., 30., 41., 32., 29., 42., 1.,
        28., 4., 36., 19., 33., 18., 34., 2., 33., 11., 20., 20., 28., 22., 31., 29., 34., 32., 38.,
        30., 49., 31., 40., 32., 45., 38., 39., 30., 47., 34., 44., 31., 43., 33., 41., 34., 39., 32.,
        36., 37., 42., 37., 41., 43., 38., 40., 38., 39., 35., 38., 37., 40., 36., 38., 2., 38., 38.,
        32., 20., 20., 31., 27., 29., 28., 26., 29., 32., 31., 35., 38., 32., 38., 32., 31., 27., 37.,
        27., 38., 29., 39., 30., 35., 32., 33., 32., 34., 39., 37., 47., 37., 52., 41., 46., 36., 44.,
        42., 40., 44., 34., 44., 39., 36., 40., 37., 39., 5., 20., 20., 29., 29., 31., 33., 32., 29.,
        32., 33., 27., 33., 34., 32., 35., 37., 33., 34., 37., 39., 5., 31., 33., 30., 33., 30., 34.,
        31., 35., 4., 38., 30., 35., 29., 34., 28., 42., 30., 34., 26., 36., 26., 30., 32., 33., 2.,
        32., 24., 30., 30., 33., 24., 34., 22., 30., 20., 32., 22., 35., 25., 32., 28., 34., 27., 32.,
        27., 2., 27., 3., 12., 35., 13., 35., 9., 37., 20., 20., 29., 29., 27., 28., 28., 28., 29.,
        32., 38., 28., 40., 27., 45., 31., 40., 34., 42., 31., 35., 42., 37., 39., 38., 38., 42., 39.,
        39., 38., 42., 36., 41., 43., 34., 26., 41., 25., 43., 26., 44., 33., 39., 31., 39., 28., 38.,
        6., 44., 0., 20., 20., 31., 28., 29., 47., 5., 43., 20., 45., 23., 45., 32., 37., 2., 22., 34.,
        23., 32., 26., 31., 29., 29., 32., 30., 30., 26., 25., 4., 33., 26., 31., 30., 35., 30., 35.,
        4., 35., 29., 38., 35., 30., 31., 36., 32., 36., 31., 39., 19., 32., 16., 31., 16., 32., 16.,
        32., 17., 32., 4., 32., 15., 31., 16., 32., 14., 35., 14., 33., 13., 29., 13., 7., 19., 26.,
        16., 27., 12., 27., 13., 25., 4., 32., 10., 5., 13., 24., 10., 23., 15., 21., 13., 22., 6.,
        27., 12., 20., 20., 29., 29., 27., 33., 37., 31., 39., 31., 42., 31., 34., 29., 33., 42., 34.,
        38., 40., 6., 33., 6., 38., 35., 40., 38., 37., 36., 45., 35., 48., 40., 46., 43., 43., 41.,
        52., 5., 38., 27., 29., 21., 27., 24., 36., 3., 24., 19., 0., 20., 20., 27., 27., 33., 28.,
        33., 34., 8., 37., 34., 37., 35., 40., 34., 37.];

    let mut quantile = Quantile::new(0.999);

    for &o in observations.iter() {
        quantile.add(o);
    }

    let _ = quantile.quantile();
}
