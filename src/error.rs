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
use std::sync::Mutex;
use num_traits::float::Float;
use super::Dirty;
use nanpack::NanPack;

#[derive(Debug, Clone)]
pub struct FloatError {
    msg: String,
}

#[cfg(not(build = "release"))]
lazy_static! {
    static ref FLOAT_ERRORS: Mutex<Vec<FloatError>> = Mutex::new(Vec::new());
}

impl FloatError {
    pub fn new(msg: &str) -> Self {
        Self { msg: msg.into() }
    }

    #[cfg(not(build = "release"))]
    pub fn new_debug<F: Float + NanPack<usize>>(msg: &str) -> Dirty<F> {
        let mut errors = FLOAT_ERRORS.lock().unwrap();
        let err_index = errors.len();
        errors.push(FloatError::new(msg));
        Dirty::new(NanPack::set_payload(err_index))
    }

    pub fn from_err_no(err_no: usize) -> FloatError {
        let errors = FLOAT_ERRORS.lock().unwrap();
        errors[err_no].clone()
    }
}

impl Error for FloatError {
    fn description(&self) -> &str {
        "nan value encountered while sanitizing dirty float"
    }
}

impl fmt::Display for FloatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn try_sanitize() {
        let good: Dirty<f64> = 1.0.into();
        let bad: Dirty<f64> = ::std::f64::NAN.into();
        assert!(good.sanitize().is_ok());
        assert!(bad.sanitize().is_err());
    }


    #[test]
    fn bad_operation() {
        let a = F64::try_new(0.0).unwrap();
        let b = F64::try_new(0.0).unwrap();
        let c = a / b;

        assert_eq!(
            c.sanitize().err().unwrap().msg,
            "got NaN from operation 0 / 0"
        );
        assert_eq!(
            F64::try_new(std::f64::NAN).err().unwrap().msg,
            "tried to sanitize NaN"
        );

    }


}
