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

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BoundedFloatError {
    NaN,
    PosInf,
    NegInf,
}

impl Error for BoundedFloatError {
    fn description(&self) -> &str {
        "invalid value encountered while sanitizing dirty bounded float"
    }
}

impl fmt::Display for BoundedFloatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BoundedFloatError::NaN => write!(f, "invalid bounded float value: NaN (not a number)"),
            BoundedFloatError::PosInf => write!(f, "invalid bounded float value: +∞ (+infinity)"),
            BoundedFloatError::NegInf => write!(f, "invalid bounded float value: -∞ (+infinity)"),
        }
    }
}
