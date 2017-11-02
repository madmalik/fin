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

use std::ops::{Add, Mul, Neg};
use super::{Fin, Dirty, AsRaw};

use num_traits::float::Float;

// operations

impl<B, F> Add<B> for Fin<F>
where
    F: Float,
    B: AsRaw<F>,
{
    type Output = Dirty<F>;

    fn add(self, other: B) -> Self::Output {
        Dirty::new(AsRaw::<F>::as_raw(self) + AsRaw::<F>::as_raw(other))
    }
}

impl<B, F> Add<B> for Dirty<F>
where
    F: Float,
    B: AsRaw<F>,
{
    type Output = Dirty<F>;

    fn add(self, other: B) -> Self::Output {
        Dirty::new(AsRaw::<F>::as_raw(self) + AsRaw::<F>::as_raw(other))
    }
}


impl<B, F> Mul<B> for Fin<F>
where
    F: Float,
    B: AsRaw<F>,
{
    type Output = Dirty<F>;

    fn mul(self, other: B) -> Self::Output {
        Dirty::new(AsRaw::<F>::as_raw(self) * AsRaw::<F>::as_raw(other))
    }
}

impl<B, F> Mul<B> for Dirty<F>
where
    F: Float,
    B: AsRaw<F>,
{
    type Output = Dirty<F>;

    fn mul(self, other: B) -> Self::Output {
        Dirty::new(AsRaw::<F>::as_raw(self) * AsRaw::<F>::as_raw(other))
    }
}


impl<F> Neg for Fin<F>
where
    F: Float,
{
    type Output = Fin<F>;
    fn neg(self) -> Self::Output {
        Fin::<F>::new_unchecked(-AsRaw::<F>::as_raw(self))
    }
}

impl<F> Neg for Dirty<F>
where
    F: Float,
{
    type Output = Dirty<F>;
    fn neg(self) -> Self::Output {
        Dirty::<F>::new(-AsRaw::<F>::as_raw(self))
    }
}

impl Into<Dirty<f64>> for f64 {
    fn into(self) -> Dirty<f64> {
        Dirty::<f64>::new(self)
    }
}

impl Into<Dirty<f32>> for f32 {
    fn into(self) -> Dirty<f32> {
        Dirty::<f32>::new(self)
    }
}


impl<B, F> PartialEq<B> for Fin<F>
where
    B: AsRaw<F> + Copy,
    F: Float,
    Dirty<F>: AsRaw<F>,
{
    fn eq(&self, other: &B) -> bool {
        AsRaw::<F>::as_raw(*self) == AsRaw::<F>::as_raw(*other)
    }
}

impl<B, F> PartialEq<B> for Dirty<F>
where
    B: AsRaw<F> + Copy,
    F: Float,
    Dirty<F>: AsRaw<F>,
{
    fn eq(&self, other: &B) -> bool {
        AsRaw::<F>::as_raw(*self) == AsRaw::<F>::as_raw(*other)
    }
}

impl<F> Eq for Fin<F>
where
    F: Float,
    Fin<F>: PartialEq,
{
}
