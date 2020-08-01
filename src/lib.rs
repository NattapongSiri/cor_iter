#![no_std]
//! Correlate iterators, in this crate, mean that two iterators
//! that iterate in harmony according to some pre-defined function.
//! 
//! It is different from zipping two iterators in that two zipped iterators
//! always return pair of next item from both iterators at once.
//! 
//! The correlate iterators can have `m` items return from one iterator then
//! it another `n` items return from another iterator where `m != n`.
//! 
//! It is possible to have `m == n` correlate iterators but it will be less
//! efficient than simply zip it together.
use num_traits::{identities::{one, zero}, PrimInt};

/// An enum that represent either primary's value or secondary value.
/// 
/// Primary is the iterator that is left hand side of operand.
#[derive(Debug)]
pub enum Either<T, U> {
    /// A value from primary iterator
    Primary(T),
    /// A value from secondary iterator
    Secondary(U)
}

impl<T, U> Either<T, U> {
    /// Return true if this enum contains value from primary iterator
    #[inline]
    pub fn is_primary(&self) -> bool {
        match self {
            Either::Primary(_) => true,
            _ => false
        }
    }
    /// Return true if this enum contains value from secondary iterator
    #[inline]
    pub fn is_secondary(&self) -> bool {
        match self {
            Either::Secondary(_) => true,
            _ => false
        }
    }
}

/// If both iterators return value of the same type, it'll be able to directly deref it to
/// get inner value.
impl<T> core::ops::Deref for Either<T, T> {
    type Target=T;

    fn deref(&self) -> &Self::Target {
        match self {
            Either::Primary(v) | Either::Secondary(v) => v
        }
    }
}

/// An iterator that keep return next item either in `I` or `J` depending on 
/// whether `F` return `true` or `false`.
/// If `F` return true, next item will come from `I`.
/// If `F` return false, next item will come from `J`.
/// If either of `I` or `J` is exhausted, it will consider this iterator exhausted.
/// 
/// The behavior can be summarise as table below:
/// 
/// | I | J | F |
/// |---|---|---|
/// | x | - | T |
/// | x | - | F |
/// | - | x | T |
/// | x | - | F |
/// | - | x | F |
/// | - | x | T |
/// | x | - | T |
/// | - | - | - |
/// 
/// `x` can either be in column `I` or column `J` but not both. 
/// If `x` is in in column `I`, it mean the iterator return an item from `I`. 
/// If `x` is in column `J`, it mean iterator return an item from `J`.
/// The possible value in column `F` is either `T` or `F`. 
/// `T` mean function `F` return `true`. `F` mean function `F` return `false`.
/// 
/// This iterator is one step eager. This mean that when construct, it will iterate once on primary iterator.
/// Each subsequence iteration will always have one step ahead iterate.
/// 
/// The first value always come from primary iterator. The return value from
/// function `F` only effect next item.
/// 
/// It is impossible to simulate linear correlate iterator where b is negative.
/// This is because this iterator always return one item from primary iterator first.
#[derive(Debug)]
pub struct CorIter<F, I, J> where F: FnMut(Either<&I::Item, &J::Item>) -> bool, I: Iterator, J: Iterator /*, J::Item: Clone */ {
    formula: F,
    primary: I,
    secondary: J,
    cur_i: Option<I::Item>,
    cur_j: Option<J::Item>
}

impl<F, I, J> CorIter<F, I, J> where F: FnMut(Either<&I::Item, &J::Item>) -> bool, I: Iterator, J: Iterator /*, J::Item: Clone */ {
    #[inline]
    pub fn new(formula: F, mut primary: I, secondary: J) -> CorIter<F, I, J> {
        let cur_i = primary.next();
        let cur_j = None;
        CorIter {
            formula,
            primary,
            secondary,
            cur_i,
            cur_j
        }
    }
}

impl<F, I, J> Iterator for CorIter<F, I, J>  where F: FnMut(Either<&I::Item, &J::Item>) -> bool, I: Iterator, J: Iterator {
    type Item=Either<I::Item, J::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.cur_i.take() {
            if (self.formula)(Either::Primary(&i)) {
                self.cur_i = self.primary.next();
            } else {
                self.cur_j = self.secondary.next();
            }

            Some(Either::Primary(i))
        } else if let Some(j) = self.cur_j.take() {
            if (self.formula)(Either::Secondary(&j)) {
                self.cur_i = self.primary.next();
            } else {
                self.cur_j = self.secondary.next();
            }

            Some(Either::Secondary(j))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (p_min, p_max) = self.primary.size_hint();
        let (s_min, s_max) = self.secondary.size_hint();
        // max can be guess if both primary and secondary size is known
        (p_min + s_min, p_max.and_then(|p| s_max.map(|s| s + p)))
    }
}

/// Linear correlation by number of item based on primary and secondary iterator. 
/// It take two co-efficient `a` and `b` of any subtype of `Int`. 
/// 
/// It use sign of co-efficient to determine which iterator should yield items.
/// There's two set of rules for this iterator.
/// 
/// First set of rules are:
/// - If `b > 0`, first `b` element will come from primary iterator.
/// - If `b < 0`, first `b` element, return from secondary iterator.
/// - If `b == 0`, it will proceed to below set of rules
/// 
/// After above set of rules is applied, then following rules will apply:
/// - If `a > 0`, it return `a` elements from primary then one element from secondary
/// - If `a < 0`, it return `a` elements from secondary then one element from primary.
/// - If `a == 0`, it return `None`
/// 
/// For a case where `a` = 1 and `b` = 0. It will return an interleave value between
/// primary and secondary iterator. If both iterator return value of the same kind,
/// consider interleave function from [itertools](https://crates.io/crates/itertools).
/// Another option is to use `zip` method from primary iterator. It will return
/// a pair of value from both iterator at once. It will be much more efficient than
/// evaluating each item on each iteration.
/// 
/// This iterator is lazy. It won't iterate on any of iterators until it own self
/// has been iterate.
/// 
/// # Example
/// If `a = 2`, `b = 1` then
/// 1. First item will come from primary iterator 
/// 2. Two items from primary iterator
/// 3. One item from secondary iterator
/// 4. Go back to step 2 until one of iterator return None
/// 
/// If `a = -2`, `b = 1` then
/// 1. First item will come from primary iterator 
/// 2. Two items from secondary iterator
/// 3. One item from primary iterator
/// 4. Go back to step 2 until one of iterator return None
/// 
/// If `a = 2`, `b = -1` then
/// 1. First item will come from secondary iterator 
/// 2. Two items from primary iterator
/// 3. One item from secondary iterator
/// 4. Go back to step 2 until one of iterator return None
/// 
/// If `a = -2`, `b = -1` then
/// 1. First item will come from secondaary iterator 
/// 2. Two items from secondary iterator
/// 3. One item from primary iterator
/// 4. Go back to step 2 until one of iterator return None
/// 
/// If `a = 2`, `b = 0` then
/// 1. Two items from primary iterator
/// 2. One item from secondary iterator
/// 3. Go back to step 1 until one of iterator return None
/// 
/// If `a = 0`, `b = 3` then iterator will yield only three items from primary iterator.
/// The result will be similar to `primary.take(3)` iterator but less efficient.
#[derive(Debug)]
pub struct LinearCorIter<I, J, T> 
where I: Iterator, J: Iterator, T: PrimInt {
    a: T,
    b: T,
    c: T,
    primary: I,
    secondary: J,
}

impl<I, J, T> LinearCorIter<I, J, T> where I: Iterator, J: Iterator, T: PrimInt {
    pub fn new(primary: I, secondary: J, a: T, b: T) -> LinearCorIter<I, J, T> {
        let c = if b == zero() {
            a
        } else {
            b
        };
        LinearCorIter {
            a,
            b,
            c,
            primary,
            secondary
        }
    }
}

impl<I, J, T> Iterator for LinearCorIter<I, J, T> where I: Iterator, J: Iterator, T: PrimInt {
    type Item=Either<I::Item, J::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.c > zero() {
            self.c = self.c - one();
            self.primary.next().map(|i| Either::Primary(i))
        } else if self.c < zero() {
            self.c = self.c + one();
            self.secondary.next().map(|j| Either::Secondary(j))
        } else {
            self.c = self.a;
            if self.b == zero() {
                if self.a > zero() {
                    self.secondary.next().map(|j| Either::Secondary(j))
                } else if self.a < zero() {
                    self.primary.next().map(|i| Either::Primary(i))
                } else {
                    None
                }
            } else {
                self.b = zero();
                self.next()
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (p_min, p_max) = self.primary.size_hint();
        let (s_min, s_max) = self.secondary.size_hint();
        // Max can be guess if both primary and secondary size is known.
        // More accurate guess can be made with more complex calculation based
        // on value of `a` and `b` in the future
        (p_min + s_min, p_max.and_then(|p| s_max.map(|s| s + p)))
    }
}

/// Add correlate functionalities to any sized `T` that implement `IntoIterator`.
/// The correlate mean that two iterators yield items based on some predefined rule(s).
pub trait Correlate : IntoIterator + Sized {
    /// Get an iterator that return [Either](struct.Either.html) item from this iterator or
    /// other iterator depending on number of item based on given `a` and `b` co-efficient.
    /// 
    /// See [LinearCorIter](struct.LinearCorIter.html) document for more detail on how `a` and `b` work.
    fn linear_correlate<I, T>(self, other: I, a: T, b: T) -> LinearCorIter<Self::IntoIter, I::IntoIter, T> where I: IntoIterator, T: PrimInt {
        LinearCorIter::new(self.into_iter(), other.into_iter(), a, b)
    }

    /// Return an iterator that return [Either](struct.Either.html) item from this iterator or other iterator
    /// depending on given function.
    /// 
    /// It return element from this iterator if function return true. Otherwise, it return an item
    /// from secondary iterator.
    /// See [CorIter](struct.CorIter.html) for more detail.
    fn correlate_with<I, F>(self, other: I, function: F) -> CorIter<F, Self::IntoIter, I::IntoIter> where I: IntoIterator, F: FnMut(Either<&Self::Item, &I::Item>) -> bool {
        CorIter::new(function, self.into_iter(), other.into_iter())
    }
}

impl<T> Correlate for T where T: IntoIterator {}

#[cfg(test)]
mod tests;