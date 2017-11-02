// Copyright 2017 Matthias Tellen

// Permission is hereby granted,  free of charge,  to any person  obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without restriction,
// including without  limitation  the  rights to use,  copy,  modify,  merge,  publish,  distribute,
// sublicense,  and/or sell copies of the Software,  and to permit  persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The  above  copyright  notice and this permission notice shall be included  in all copies or sub-
// stantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS",  WITHOUT WARRANTY OF ANY KIND,  EXPRESS OR IMPLIED,  INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,  FITNESS  FOR A PARTICULAR PURPOSE AND NON-
// INFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS  OR  COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAM-
// AGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,  TORT OR OTHERWISE, ARISING FROM, OUT
// OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! Working with floats can be a bit of a pain in the backside, since floats can
//! carry errors conditions (not a number and ininity from an overflow) and rust
//! does the correct thing and doesn't implement Ord.
//!
//! ...

extern crate num_traits;

mod error;
mod trait_impls;

use num_traits::float::Float;

use error::BoundedFloatError;

// two traits: BoundedFloat are all common float operatons

// AsRaw<F> for dereferencing

pub trait AsRaw<F> {
    fn as_raw(self) -> F;
}

pub trait BoundedFloat<F>
where
    F: Float,
{
    fn abs(self) -> Self;
    fn sqrt(self) -> Dirty<F>;


    #[inline]
    fn assert_sanitized(&self) {
        if cfg!(bounded_float_debug_check) {
            panic!("assertion");
        }
    }

}

#[derive(Debug, Copy, Clone)]
pub struct Fin<F: Float>(F);
#[derive(Debug, Copy, Clone)]
pub struct Dirty<F: Float>(F);


impl<F: Float> Fin<F> {
    pub fn try_new(f: F) -> Result<Fin<F>, BoundedFloatError> {
        if f.is_nan() {
            return Err(BoundedFloatError::NaN);
        }
        if f.is_infinite() {
            return Err(if f.is_sign_positive() {
                BoundedFloatError::PosInf
            } else {
                BoundedFloatError::NegInf
            });
        }
        Ok(Fin(f))
    }

    fn new_unchecked(f: F) -> Fin<F> {
        Fin(f)
    }
}

impl<F: Float> Dirty<F> {
    pub fn new(f: F) -> Dirty<F> {
        Dirty(f)
    }

    pub fn sanitize(self) -> Result<Fin<F>, BoundedFloatError> {
        Fin::try_new(self.as_raw())
    }
}

impl<F: Float> AsRaw<F> for Fin<F> {
    fn as_raw(self) -> F {
        self.0
    }
}

impl<F: Float> AsRaw<F> for Dirty<F> {
    fn as_raw(self) -> F {
        self.0
    }
}

impl<F: Float> AsRaw<F> for F {
    fn as_raw(self) -> F {
        self
    }
}

// BoundedFloat implementations
impl<F> BoundedFloat<F> for Fin<F>
where
    F: Float,
    Fin<F>: AsRaw<F>,
{
    fn abs(self) -> Self {
        Fin::new_unchecked(self.as_raw().abs())
    }

    fn sqrt(self) -> Dirty<F> {
        Dirty::new(self.as_raw().sqrt())
    }
}

impl<F> BoundedFloat<F> for Dirty<F>
where
    F: Float,
    Dirty<F>: AsRaw<F>,
{
    fn abs(self) -> Self {
        Dirty::new(self.as_raw().abs())
    }

    fn sqrt(self) -> Dirty<F> {
        Dirty::new(self.as_raw().sqrt())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::NAN;
    use std::f64::INFINITY as INF;
    type F64 = Fin<f64>;
    type DirtyF64 = Dirty<f64>;

    #[test]
    fn new() {
        assert!(F64::try_new(NAN).is_err());
        assert!(F64::try_new(INF).is_err());
        assert!(F64::try_new(1.0).is_ok());
    }

    #[test]
    fn binary_ops() {
        let a = F64::try_new(1.0).unwrap();
        let b = F64::try_new(1.0).unwrap();
        let c = DirtyF64::new(2.0);
        assert_eq!(a + b, c);
        assert_eq!(a + 1.0, 2.0);
        assert_eq!((a + b).sanitize().unwrap(), c);
    }
}
