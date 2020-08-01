// Need to use alloc crate to use `Vec` because this crate is `no-std`
extern crate alloc;

use alloc::vec::Vec;

use super::*;

/// Generate symmetric size pair of Vec where first one has value `0..=10` of type `u8`
/// and second one has value of `-10..=0` of type `i8`.
/// 
/// It is consider symmetric because two vec have same size.
fn make_symmetric_sample() -> (Vec<u8>, Vec<i8>) {
    ((0..=10).collect(), (-10..=0).collect())
}
#[test]
fn linear_corr_case01() {
    // case 1, linear cor where both `a` and `b` is 1
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0;
    let mut y_idx = 0;
    
    (&x).linear_correlate(&y, 1, 1).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx]);
                y_idx += 1;
            }
        }
        assert!(x_idx - y_idx <= 2);
    });

    assert_eq!(x_idx, 11);
    assert_eq!(y_idx, 10);
}
#[test]
fn linear_corr_case02() {
    // case 2, linear cor where both `a` and `b` is -1
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0;
    let mut y_idx = 0;
    
    (&x).linear_correlate(&y, -1, -1).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx]);
                y_idx += 1;
            }
        }
        assert!(y_idx - x_idx <= 2);
    });

    assert_eq!(x_idx, 10);
    assert_eq!(y_idx, 11);
}
#[test]
fn linear_corr_case03() {
    // case 3, linear cor where both `a` and `b` is 0
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0;
    let mut y_idx = 0;
    
    (&x).linear_correlate(&y, 0, 0).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx]);
                y_idx += 1;
            }
        }
        assert!(y_idx - x_idx <= 1);
    });

    assert_eq!(x_idx, y_idx);
    assert_eq!(x_idx, 0);
}
#[test]
fn linear_corr_case04() {
    // case 4, linear cor where both `a` is 1 and `b` is 0
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0;
    let mut y_idx = 0;
    
    (&x).linear_correlate(&y, 1, 0).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx]);
                y_idx += 1;
            }
        }
        assert!(x_idx - y_idx <= 1);
    });

    assert_eq!(x_idx, y_idx);
    assert_eq!(x_idx, 11);
}
#[test]
fn linear_corr_case05() {
    // case 5, linear cor where both `a` is -1 and `b` is 0
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0;
    let mut y_idx = 0;
    
    (&x).linear_correlate(&y, -1, 0).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx]);
                y_idx += 1;
            }
        }
        assert!(y_idx - x_idx <= 1);
    });

    assert_eq!(x_idx, y_idx);
    assert_eq!(x_idx, 11);
}
#[test]
fn linear_corr_case06() {
    // case 6, linear cor where both `a` is -1 and `b` is 1
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0isize;
    let mut y_idx = 0isize;
    
    (&x).linear_correlate(&y, -1, 1).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx as usize]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx as usize]);
                y_idx += 1;
            }
        }
        assert!(x_idx - y_idx <= 1);
    });

    assert_eq!(x_idx, y_idx);
    assert_eq!(x_idx, 11);
}
#[test]
fn linear_corr_case07() {
    // case 7, linear cor where both `a` is 1 and `b` is -1
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0isize;
    let mut y_idx = 0isize;
    
    (&x).linear_correlate(&y, 1, -1).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx as usize]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx as usize]);
                y_idx += 1;
            }
        }
        assert!(y_idx - x_idx <= 1);
    });

    assert_eq!(x_idx, y_idx);
    assert_eq!(x_idx, 11);
}
#[test]
fn linear_corr_case08() {
    // case 8, linear cor where both `a` is 2 and `b` is 0
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0;
    let mut y_idx = 0;
    
    (&x).linear_correlate(&y, 2, 0).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx]);
                y_idx += 1;
            }
        }
        assert!((x_idx as f64 / 2f64) as usize - y_idx <= 1);
    });

    assert_eq!(x_idx, 11);
    assert_eq!(y_idx, 5);
}
#[test]
fn linear_corr_case09() {
    // case 9, linear cor where both `a` is -2 and `b` is 0
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0;
    let mut y_idx = 0;
    
    (&x).linear_correlate(&y, -2, 0).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx]);
                y_idx += 1;
            }
        }
        assert!((y_idx as f64 / 2f64) as usize - x_idx <= 1);
    });

    assert_eq!(y_idx, 11);
    assert_eq!(x_idx, 5);
}
#[test]
fn linear_corr_case10() {
    // case 10, linear cor where both `a` is 2 and `b` is 1
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0;
    let mut y_idx = 0;
    
    (&x).linear_correlate(&y, 2, 1).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx]);
                y_idx += 1;
            }
        }
        assert!((x_idx as f64 / 2f64) as usize - y_idx <= 1);
        if x_idx < 3 {
            assert_eq!(y_idx, 0);
        }
    });

    assert_eq!(x_idx, 11);
    assert_eq!(y_idx, 5);
}
#[test]
fn linear_corr_case11() {
    // case 11, linear cor where both `a` is 2 and `b` is -1
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0isize;
    let mut y_idx = 0isize;
    
    (&x).linear_correlate(&y, 2, -1).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx as usize]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx as usize]);
                y_idx += 1;
            }
        }
        assert!((x_idx as f64 / 2f64) as isize - y_idx <= 1);
        if x_idx < 2 {
            assert_eq!(y_idx, 1);
        }
    });

    assert_eq!(x_idx, 11);
    assert_eq!(y_idx, 6);
}
#[test]
fn linear_corr_case12() {
    // case 12, linear cor where both `a` is -2 and `b` is 1
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0isize;
    let mut y_idx = 0isize;
    
    (&x).linear_correlate(&y, -2, 1).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx as usize]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx as usize]);
                y_idx += 1;
            }
        }
        assert!((y_idx as f64 / 2f64) as isize - x_idx <= 1);
        if y_idx < 2 {
            assert_eq!(x_idx, 1);
        }
    });

    assert_eq!(y_idx, 11);
    assert_eq!(x_idx, 6);
}
#[test]
fn linear_corr_case13() {
    // case 13, linear cor where both `a` is -2 and `b` is -1
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0isize;
    let mut y_idx = 0isize;
    
    (&x).linear_correlate(&y, -2, -1).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx as usize]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx as usize]);
                y_idx += 1;
            }
        }
        assert!((y_idx as f64 / 2f64) as isize - x_idx <= 1);
        if y_idx < 3 {
            assert_eq!(x_idx, 0);
        }
    });

    assert_eq!(y_idx, 11);
    assert_eq!(x_idx, 5);
}
#[test]
fn linear_corr_case14() {
    // case 14, linear cor where both `a` is out of bound and `b` is 1
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0isize;
    let mut y_idx = 0isize;
    
    (&x).linear_correlate(y.iter(), x.len() + 1, 1).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx as usize]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx as usize]);
                y_idx += 1;
            }
        }
        if (x_idx as usize) < x.len() {
            assert_eq!(y_idx, 0);
        }
    });

    assert_eq!(x_idx, 11);
    assert_eq!(y_idx, 0);
}
#[test]
fn linear_corr_case15() {
    // case 15, linear cor where both `a` is 1 and `b` is out of bound
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0isize;
    let mut y_idx = 0isize;
    
    (&x).linear_correlate(&y, 1, x.len() + 1).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx as usize]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx as usize]);
                y_idx += 1;
            }
        }
        if (x_idx as usize) < x.len() {
            assert_eq!(y_idx, 0);
        }
    });

    assert_eq!(x_idx, 11);
    assert_eq!(y_idx, 0);
}
#[test]
fn linear_corr_case16() {
    // case 16, linear cor where both `a` is 1 and `b` is negative out of bound
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0isize;
    let mut y_idx = 0isize;
    
    (&x).linear_correlate(&y, 1, y.len() as isize * -1 - 1).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx as usize]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx as usize]);
                y_idx += 1;
            }
        }
        if (y_idx as usize) < y.len() {
            assert_eq!(x_idx, 0);
        }
    });

    assert_eq!(y_idx, 11);
    assert_eq!(x_idx, 0);
}
#[test]
fn linear_corr_case17() {
    // case 17, linear cor where both `a` is negative out of bound and `b` is 1
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0isize;
    let mut y_idx = 0isize;
    
    (&x).linear_correlate(&y, y.len() as isize * -1 - 1, 1).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx as usize]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx as usize]);
                y_idx += 1;
            }
        }
        if (y_idx as usize) < y.len() {
            assert_eq!(x_idx, 1);
        }
    });

    assert_eq!(y_idx, 11);
    assert_eq!(x_idx, 1);
}
#[test]
fn corr_with_case01() {
    // case 1, interleaving between primary and secondary
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0;
    let mut y_idx = 0;
    
    x.iter().correlate_with(&y, |v| {
        v.is_secondary()
    }).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx]);
                y_idx += 1;
            }
        }
        assert!(x_idx - y_idx <= 1);
    });

    assert_eq!(x_idx, y_idx);
    assert_eq!(x_idx, x.len());
}
#[test]
fn corr_with_case02() {
    // case 2, consume only primary
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0;
    let mut y_idx = 0;
    
    x.iter().correlate_with(&y, |_| {
        true
    }).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx]);
                y_idx += 1;
            }
        }
    });

    assert_eq!(x_idx, x.len());
    assert_eq!(y_idx, 0);
}
#[test]
fn corr_with_case03() {
    // case 3, consume only secondary
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0;
    let mut y_idx = 0;
    
    x.iter().correlate_with(&y, |_| {
        false
    }).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx]);
                y_idx += 1;
            }
        }
    });

    // Since correlate_with always consume value from primary at least once, x_idx must be 1.
    assert_eq!(x_idx, 1);
    assert_eq!(y_idx, y.len());
}
#[test]
fn corr_with_case04() {
    // case 4, consume first 6 of both iterators then interleave iterate afterward
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0;
    let mut y_idx = 0;
    
    x.iter().correlate_with(&y, |v| {
        match v {
            Either::Primary(p) => {
                **p < 5 // when current p is >= 5, it mean that it's consume up to this **p value
            },
            Either::Secondary(s) => {
                **s > -5 // when current p is > -5, it mean that it's consume up to this **p value
            }
        }
    }).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx]);
                x_idx += 1;
            },
            Either::Secondary(v) => {
                assert_eq!(*v, y[y_idx]);
                y_idx += 1;
            }
        }

        if x_idx <= 5 {
            assert_eq!(y_idx, 0);
        }
    });

    assert_eq!(x_idx, x.len());
    assert_eq!(y_idx, y.len());
}
#[test]
fn corr_with_case05() {
    // case 5, simulate linear correlation where a = -2 and b = 1. 
    let (x, y) = make_symmetric_sample();
    let mut x_idx = 0;
    let mut y_idx = 0;
    
    x.iter().correlate_with(y.iter().enumerate(), |v| {
        match v {
            Either::Primary(_) => {
                false // after it return one item from primary, next item always come from secondary
            },
            Either::Secondary((i, _)) => {
                i % 2 == 1 // enumerate begin with 0 so if it mod 2 == 1 then it mean two item is returned
            }
        }
    }).for_each(|either| {
        match either {
            Either::Primary(v) => {
                assert_eq!(*v, x[x_idx]);
                x_idx += 1;
            },
            Either::Secondary((_, v)) => {
                assert_eq!(*v, y[y_idx]);
                y_idx += 1;
            }
        }

        if x_idx == 1 {
            assert!(y_idx <= 2);
        } else {
            assert!(x_idx - (y_idx as f32 / 2f32) as usize <= 1);
        }
    });

    assert_eq!(x_idx, 6);
    assert_eq!(y_idx, y.len());
}