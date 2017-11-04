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

use error::SanitizeFloatError;

pub type F64 = Fin<f64>;
pub type DirtyF64 = Dirty<f64>;

pub type F32 = Fin<f32>;
pub type DirtyF32 = Dirty<f32>;

pub trait UncheckedConv<F>
where
    Self: Sized,
{
    fn as_raw(self) -> F;

    #[inline]
    fn map<C: Fn(F) -> F>(self, c: C) -> Self {
        UncheckedConv::from_raw(c(self.as_raw()))
    }

    fn from_raw(F) -> Self;
}

// some macro helpers to replicate all the methods for FinFloat from Float
macro_rules! non_tainting_method {
    ($method_name: ident) => {
        #[inline]
            fn $method_name(self) -> Self {
                self.map(Float::$method_name)
            }
    };
    ($method_name: ident, rhs) => {
        #[inline]
            fn $method_name<O: UncheckedConv<F> + Copy>(self, o: O) -> Self {
                self.map(|x| Float::$method_name(x, o.as_raw()))
            }
    }
}

macro_rules! tainting_method {
    ($method_name: ident) => {
        #[inline]
            fn $method_name(self) -> Self {
                self.map(Float::$method_name)
            }
    };
}

pub trait FinFloat<F>
where
    F: Float,
    Self: Sized + UncheckedConv<F>,
{
    non_tainting_method!(floor);
    non_tainting_method!(ceil);
    non_tainting_method!(round);
    non_tainting_method!(fract);
    non_tainting_method!(abs);
    non_tainting_method!(signum);
    non_tainting_method!(to_radians);
    non_tainting_method!(cbrt);
    non_tainting_method!(hypot, rhs);
    non_tainting_method!(sin);
    non_tainting_method!(cos);
    non_tainting_method!(tan);
    non_tainting_method!(atan);
    non_tainting_method!(atan2, rhs);
    non_tainting_method!(tanh);

    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        let (s, c) = self.as_raw().sin_cos();
        (UncheckedConv::from_raw(s), UncheckedConv::from_raw(c))
    }

    #[inline]
    fn mul_add<A: UncheckedConv<F> + Copy, B: UncheckedConv<F> + Copy>(
        self,
        a: A,
        b: B,
    ) -> Dirty<F> {
        Dirty::new(self.as_raw().mul_add(a.as_raw(), b.as_raw()))
    }
    tainting_method!(recip);
    #[inline]
    fn powi(self, exp: i32) -> Dirty<F> {
        Dirty::new(self.as_raw().powi(exp))
    }
    #[inline]
    fn powf<A: UncheckedConv<F> + Copy>(self, exp: A) -> Dirty<F> {
        Dirty::new(self.as_raw().powf(exp.as_raw()))
    }

    tainting_method!(sqrt);
    tainting_method!(exp);
    tainting_method!(exp2);
    tainting_method!(ln);

    #[inline]
    fn log<A: UncheckedConv<F> + Copy>(self, a: A) -> Dirty<F> {
        Dirty::new(self.as_raw().log(a.as_raw()))
    }

    tainting_method!(log2);
    tainting_method!(log10);
    tainting_method!(to_degrees);
    tainting_method!(acos);
    tainting_method!(asin);
    tainting_method!(exp_m1);
    tainting_method!(ln_1p);
    tainting_method!(sinh);
    tainting_method!(cosh);
    tainting_method!(asinh);
    tainting_method!(acosh);
    tainting_method!(atanh);

    #[inline]
    fn taint(self) -> Dirty<F> {
        Dirty::<F>::new(self.as_raw())
    }

    // TODO
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

impl<F> FinFloat<F> for Fin<F>
where
    F: Float,
    Fin<F>: UncheckedConv<F>,
{
}

impl<F> FinFloat<F> for Dirty<F>
where
    F: Float,
    Dirty<F>: UncheckedConv<F>,
{
}

impl<F: Float> Fin<F> {
    #[inline]
    pub fn try_new(f: F) -> Result<Fin<F>, SanitizeFloatError> {
        if f.is_infinite() {
            return Err(if f.is_sign_positive() {
                SanitizeFloatError::PosInf
            } else {
                SanitizeFloatError::NegInf
            });
        }

        if f.is_nan() {
            return Err(SanitizeFloatError::NaN);
        }

        Ok(Fin::from_raw(f))
    }
}

impl<F: Float> Dirty<F> {
    #[inline]
    pub fn new(f: F) -> Dirty<F> {
        Dirty(f)
    }

    #[inline]
    pub fn sanitize(self) -> Result<Fin<F>, SanitizeFloatError> {
        Fin::try_new(self.as_raw())
    }
}

impl<F: Float> UncheckedConv<F> for Fin<F> {
    #[inline]
    fn as_raw(self) -> F {
        self.0
    }

    #[inline]
    fn from_raw(f: F) -> Self {
        Fin(f)
    }
}

impl<F: Float> UncheckedConv<F> for Dirty<F> {
    #[inline]
    fn as_raw(self) -> F {
        self.0
    }

    #[inline]
    fn from_raw(f: F) -> Self {
        Dirty(f)
    }
}

impl<F: Float> UncheckedConv<F> for F {
    #[inline]
    fn as_raw(self) -> F {
        self
    }

    #[inline]
    fn from_raw(f: F) -> Self {
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::NAN;
    use std::f64::INFINITY as INF;

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
