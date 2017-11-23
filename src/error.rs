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

use std::fmt;
use std::sync::Mutex;
use num_traits::float::Float;
use std::collections::BTreeMap;
use std::num::FpCategory;
use backtrace;


#[cfg(not(build = "release"))]
lazy_static! {
    pub(crate) static ref FLOAT_ERROR_BUFFER: ErrorBuffer = Default::default();
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum FloatClass {
    PlusZero,
    MinusZero,
    PlusInfinity,
    MinusInfinity,
    NaN,
    Other,
}

impl<F> From<F> for FloatClass
where
    F: Float,
{
    fn from(f: F) -> Self {
        match (f.classify(), f.is_sign_positive()) {
            (FpCategory::Infinite, true) => FloatClass::PlusInfinity,
            (FpCategory::Infinite, false) => FloatClass::MinusInfinity,
            (FpCategory::Zero, true) => FloatClass::PlusZero,
            (FpCategory::Zero, false) => FloatClass::MinusZero,
            (FpCategory::Nan, _) => FloatClass::NaN,
            _ => FloatClass::Other,
        }
    }
}


impl fmt::Display for FloatClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FloatClass::PlusZero => write!(f, "zero"),
            FloatClass::MinusZero => write!(f, "negative zero"),
            FloatClass::PlusInfinity => write!(f, "infinity"),
            FloatClass::MinusInfinity => write!(f, "negative infinity"),
            FloatClass::NaN => write!(f, "NaN"),
            FloatClass::Other => write!(f, "value"),
        }
    }
}

// There is just one bucket right now. If this proves to be a bottleneck, it's
// possible to switch to mulitple buckets. That would work like this: Say, we
// use 16 buckets. These live in an array. When a new error is created, one
// bucket is choosen at random, the lock is aquired and the error is stored.
// To make the index unique and the bucket identifiyable via the index, it's
// shifted 4 bits to the left and the bucket index is safed in these 4 bits.
pub(crate) struct ErrorBuffer {
    bucket: Mutex<ErrorBufferBucket>,
}

pub(crate) struct ErrorBufferBucket {
    // is incremented for every new error
    index: usize,
    errors: BTreeMap<usize, FloatError>,
}

impl ErrorBuffer {
    pub(crate) fn insert(&self, error: FloatError) -> usize {
        let mut bucket = self.bucket.lock().unwrap();
        bucket.index += 1;
        let index = bucket.index;
        bucket.errors.insert(index, error);
        bucket.index
    }

    pub(crate) fn remove(&self, index: usize) -> FloatError {
        let mut bucket = self.bucket.lock().unwrap();
        bucket.errors.remove(&index).expect("error in error buffer")
    }
}

impl Default for ErrorBuffer {
    fn default() -> Self {
        ErrorBuffer {
            bucket: Mutex::new(ErrorBufferBucket {
                index: 0,
                errors: BTreeMap::new(),
            }),
        }
    }
}

#[derive(Fail, PartialEq)]
#[fail(display = "{}: {}", debug_info, variant)]
pub struct FloatError {
    debug_info: DebugInfo,
    variant: FloatErrorInner,
}

impl fmt::Debug for FloatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Fail, Debug, PartialEq)]
pub(crate) enum FloatErrorInner {
    #[fail(display = "Division {} by {} resulted in NaN", a, b)]
    Div { a: FloatClass, b: FloatClass },
    #[fail(display = "Multiplication {} by {} resulted in NaN", a, b)]
    Mul { a: FloatClass, b: FloatClass },
    #[fail(display = "Sanitization of {}", a)]
    Sanitization { a: FloatClass },
}

#[derive(Debug, PartialEq)]
pub(crate) struct DebugInfo {
    lineno: u32,
    filename: String,
}

impl fmt::Display for DebugInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.filename, self.lineno)
    }
}

fn get_caller_debug_info(mut depth: usize) -> DebugInfo {
    let mut debug_info = DebugInfo {
        lineno: 0,
        filename: String::new(),
    };
    backtrace::trace(|frame| {
        if depth == 1 {
            let ip = frame.ip();
            backtrace::resolve(ip, |symbol| {
                if let Some(s) = symbol.filename().and_then(|f| f.to_str()) {
                    debug_info.filename.push_str(s);
                }
                if let Some(l) = symbol.lineno() {
                    debug_info.lineno = l;
                }
            });
            return false;
        }
        depth -= 1;
        true
    });
    debug_info
}
const STACKTRACE_DEPTH: usize = 5;

impl FloatError {
    #[cfg(not(build = "release"))]
    pub(crate) fn div<F: Into<FloatClass>>(a: F, b: F) -> Self {
        FloatError {
            debug_info: get_caller_debug_info(STACKTRACE_DEPTH),
            variant: FloatErrorInner::Div {
                a: a.into(),
                b: b.into(),
            },
        }
    }

    pub(crate) fn mul<F: Into<FloatClass>>(a: F, b: F) -> Self {
        FloatError {
            debug_info: get_caller_debug_info(STACKTRACE_DEPTH),
            variant: FloatErrorInner::Mul {
                a: a.into(),
                b: b.into(),
            },
        }
    }


    pub(crate) fn sanitization<F: Into<FloatClass>>(a: F) -> Self {
        FloatError {
            debug_info: get_caller_debug_info(STACKTRACE_DEPTH),
            variant: FloatErrorInner::Sanitization { a: a.into() },
        }
    }


    /*
    pub fn new() -> Self {
        // creates the error with message `msg`. `origin` is the file name of the caller, used to
        // find the stack frame of the caller of that.

        let mut filename = None;
        let mut lineno = None;

        // the bools `catch_next_frame` and `next` are used as for control flow, since we cannot
        // return from nested closures
        let mut catch_next_frame = false;

        backtrace::trace(|frame| {
            let ip = frame.ip();
            let mut next = true;

            backtrace::resolve(ip, |symbol| {
                let curr_filename = symbol.filename().and_then(|f| f.to_str());

                if catch_next_frame {
                    // this means the last iteration has found the stack frame of the operation
                    lineno = symbol.lineno();
                    filename = curr_filename.map(|s| s.into());
                    next = false;
                } else {
                    // This is a super ugly hack to find the stack of the operation that caused the
                    // the error. The frame after this is one we are after.
                    // If the filename cannot be read, the whole iteration falls through and
                    // `filenamo` and `lineno` stay `None`
                    catch_next_frame = curr_filename == Some(origin);
                }
            });

            next
        });

        Self {
            msg: msg.into(),
            lineno,
            filename,
        }
    }
    #[cfg(not(build = "release"))]
    pub fn new_debug<F: Float + NanPack<usize>>(msg: &str, origin: &str) -> Dirty<F> {
        let mut errors = FLOAT_ERRORS.lock().unwrap();
        let err_index = errors.len();
        errors.push(FloatError::new(msg, origin));
        Dirty::new(NanPack::set_payload(err_index))
    }

    #[cfg(not(build = "release"))]
    pub fn from_err_no(err_no: usize) -> FloatError<F> {
        let errors = FLOAT_ERRORS.lock().unwrap();
        errors[err_no].clone()
    }
*/
}



#[cfg(test)]
mod tests {
    use super::super::*;
    use super::{FloatClass, FloatErrorInner};


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

        let err = c.sanitize().err().unwrap();
        assert_eq!(
            FloatErrorInner::Div {
                a: FloatClass::PlusZero,
                b: FloatClass::PlusZero,
            },
            err.variant
        );


        let err = F64::try_new(std::f64::NAN).err().unwrap();
        assert_eq!(
            FloatErrorInner::Sanitization { a: FloatClass::NaN },
            err.variant
        );

    }


}
