//! Tiny module for counting non-pseudoknotted secondary structures.

extern crate num_bigint;
extern crate num_traits;

use cached::proc_macro::cached;
use num_bigint::BigUint;
use num_traits::One;
use std::ops::{Add, MulAssign};

#[cached]
fn _count_structures(n: i64, mingap: i64) -> BigUint {
    let mut v: BigUint = One::one();
    if n > mingap {
        v = _count_structures(n - 1, mingap);
        for k in 1..n - mingap {
            let mut w = _count_structures(k - 1, mingap);
            w.mul_assign(_count_structures(n - k - 1, mingap));
            v = v.add(w);
        }
    }
    v
}

/// Returns the count of possible non-pseudoknotted secondary structures of a specified length, n,
/// with at least 'mingap' unpaired nucleotides between every base-pair.
pub fn count_structures(n: i64, mingap: i64) -> BigUint {
    for i in 1..n + 1 {
        _count_structures(i, mingap);
    }
    _count_structures(n, mingap)
}