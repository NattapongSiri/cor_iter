# cor_iter
A correlate iterators where two iterators take turn to return different number of item.

# How to use
Put following line into `cargo.toml`
```toml
cor_iter = "*"
```

Use trait `Correlate` and enum `Either`.
```rust
use cor_iter::{Correlate, Either};
```

It will provide two methods to anything that implement `core::iter::IntoIterator`.

First method is `linear_correlate`.
Parameter `a` determine the number of objects to be returns before a value from another
iterator is return. If `a` is positive, it mean the number of primary iterator. 
If `a` is negative, it mean the number of secondary iterator.

Parameter `b` determine the first absolute value of `b` items will be return from one of iterator.
If `b` is positive, it mean first `abs(b)` item came from primary iterator.
If `b` is negative, it mean first `abs(b)` item came from secondary iterator.
```rust
// First 12 object came from `vec_obj1` because `b` is 2 and `a` is 10.
// Object 13 came from `vec_obj2`. Object 14 - 24 then came from `vec_obj1`
// then object 25 came from `vec_obj2`. Object 26 - 36 then came from `vec_obj1` and so on.
vec_obj1.linear_correlate(vec_obj2, 10, 2).for_each(|result| {
    match result {
        Either::Primary(p) => {
            // do something with object from `vec_obj1`
        },
        Either::Secondary(s) => {
            // do something with obj from `vec_obj2`
        }
    }
});
```

Second method is `correlate_with`
It take a closure that if it return `true`, next value will come from primary iterator.
If a closure return `false`, next value will come from secondary iterator.
```rust
// Keep iterate on primary iterator while the value is less than 5 then it yield from secondary iterator.
// It will keep iterate on secondary iterator until value is greater than -5 then it yield from primary iterator.
vec_1.iter().correlate_with(&vec_2, |current| {
    match current {
        Either::Primary(p) => {
            **p < 5 // when current p is >= 5, it mean that it's consume up to this **p value
        },
        Either::Secondary(s) => {
            **s > -5 // when current p is > -5, it mean that it's consume up to this **p value
        }
    }
}).for_each(|result| {
    match result {
        Either::Primary(p) => {
            // do something with value from vec_1
        },
        Either::Secondary(s) => {
            // do something with value from vec_2
        }
    }
});
```

# Caveat
`correlate_with` method will return an iterator whose first value will always came from left hand side iterator. Unlike `linear_correlate` where negative `b` make the first `b` value of iterator come from right hand side.