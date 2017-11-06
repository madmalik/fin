// Copyright 2017 Matthias Tellen
//
// Permission is hereby granted,  free of charge,  to any person  obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without restriction,
// including without  limitation  the  rights to use,  copy,  modify,  merge,  publish,  distribute,
// sublicense,  and/or sell copies of the Software,  and to permit  persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The  above  copyright  notice and this permission notice shall be included  in all copies or sub-
// stantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS",  WITHOUT WARRANTY OF ANY KIND,  EXPRESS OR IMPLIED,  INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,  FITNESS  FOR A PARTICULAR PURPOSE AND NON-
// INFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS  OR  COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAM-
// AGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,  TORT OR OTHERWISE, ARISING FROM, OUT
// OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::ops::{Add, Mul, Div, Neg};
use std::cmp::Ordering;
use std::fmt;
use super::{Fin, Dirty, UncheckedConv};
use super::error::FloatError;
use super::nanpack::{NanPack, GetPayloadResult};

use num_traits::float::Float;

macro_rules! impl_common_traits {
    ( $( $name: path),* ) => {
        $(
            impl<B, F> Add<B> for $name
            where
                F: Float,
                B: UncheckedConv<F>,
            {
                type Output = Dirty<F>;

                #[inline]
                fn add(self, other: B) -> Self::Output {
                    Dirty::from_raw(self.as_raw() + other.as_raw())
                }
            }

            impl<B, F> Mul<B> for $name
            where
                F: Float,
                B: UncheckedConv<F>,
            {
                type Output = Dirty<F>;

                #[inline]
                fn mul(self, other: B) -> Self::Output {
                    Dirty::from_raw(self.as_raw() * other.as_raw())
                }
            }

            impl<B, F> Div<B> for $name
            where
                F: Float + NanPack<usize> + fmt::Debug,
                B: UncheckedConv<F> + Copy,
            {
                type Output = Dirty<F>;

                #[inline]
                fn div(self, other: B) -> Self::Output {
                    let result = self.as_raw() / other.as_raw();
                    #[cfg(not(build = "release"))]
                    {
                        if let GetPayloadResult::EmptyNan = result.get_payload() {
                            // new error, report it
                            return FloatError::new_debug(&format!("got NaN from operation {:?} / {:?}", self.as_raw(), other.as_raw()));
                        }
                    }
                    Dirty::from_raw(result)
                }
            }

            impl<F> Neg for $name
            where
                F: Float,
            {
                type Output = $name;
                #[inline]
                fn neg(self) -> Self::Output {
                    Self::from_raw(-self.as_raw())
                }
            }

            impl<B, F> PartialEq<B> for $name
            where
                B: UncheckedConv<F> + Copy,
                F: Float,
            {
                #[inline]
                fn eq(&self, other: &B) -> bool {
                    self.as_raw() == other.as_raw()
                }
            }

            impl<B, F> PartialOrd<B> for $name
            where
                B: UncheckedConv<F> + Copy,
                F: Float,
            {
                #[inline]
                fn partial_cmp(&self, other: &B) -> Option<Ordering> {
                    self.as_raw().partial_cmp(&other.as_raw())
                }

                #[inline]
                fn lt(&self, other: &B) -> bool {
                    self.as_raw().lt(&other.as_raw())
                }

                #[inline]
                fn le(&self, other: &B) -> bool {
                    self.as_raw().le(&other.as_raw())
                }

                #[inline]
                fn gt(&self, other: &B) -> bool {
                    self.as_raw().gt(&other.as_raw())
                }

                #[inline]
                fn ge(&self, other: &B) -> bool {
                    self.as_raw().ge(&other.as_raw())
                }
            }
        )*
    }
}

impl_common_traits!(Fin<F>, Dirty<F>);

impl Into<Dirty<f64>> for f64 {
    fn into(self) -> Dirty<f64> {
        Dirty::<f64>::from_raw(self)
    }
}

impl Into<Dirty<f32>> for f32 {
    fn into(self) -> Dirty<f32> {
        Dirty::<f32>::from_raw(self)
    }
}

impl<F> Eq for Fin<F>
where
    F: Float,
    Fin<F>: PartialEq,
{
}


impl<F> Ord for Fin<F>
where
    F: Float,
{
    fn cmp(&self, other: &Fin<F>) -> Ordering {
        let a = self.as_raw();
        let b = other.as_raw();

        if a < b {
            Ordering::Less
        } else if a == b {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl<F> fmt::Display for Fin<F>
where
    F: Float + fmt::Display,
    Self: UncheckedConv<F>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (*self).as_raw())
    }
}

impl<F> fmt::Display for Dirty<F>
where
    F: Float + fmt::Display,
    Self: UncheckedConv<F>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (*self).as_raw())
    }
}
