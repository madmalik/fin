// We're using manual transmutes instead of the to- and from_bits() methods to preserve the sNaNs
#![cfg_attr(feature = "cargo-clippy", allow(transmute_int_to_float))]
use std::mem::transmute;

const F32_PAYLOAD_MASK: u32 = 0x1F_FFFF;
const F32_EMPTY_NAN: u32 = 0x_7fc0_0000;

const F64_PAYLOAD_MASK: u64 = 0x3_FFFF_FFFF_FFFF;
const F64_EMPTY_NAN: u64 = 0x7ff8_0000_0000_0000;

pub trait NanPack<T> {
    fn set_payload(T) -> Self;
    fn is_payloaded(self) -> bool;
    fn get_payload(self) -> Option<T>;
}

// just a little macro to avoid repeating the same stuff for f64 and f32
macro_rules! impl_NanPack {
    ( $f: ty, $u: ty, $payload_mask: ident, $empty_nan: ident) => {
        impl NanPack<usize> for $f {
            fn set_payload(val: usize) -> Self {
                let val = (val + 1) as $u;
                assert!(val <= $payload_mask);
                unsafe { transmute(val | $empty_nan) }
            }

            fn is_payloaded(self) -> bool {
                let bits: $u = unsafe { transmute(self) };
                self.is_nan() && ((bits & $payload_mask) > 0)
            }

            fn get_payload(self) -> Option<usize> {
                if !self.is_payloaded() {
                    return None;
                }
                let bits: $u = self.to_bits();
                let payload = bits & $payload_mask;
                Some((payload - 1) as usize)
            }
        }
    }
}

impl_NanPack!(f64, u64, F64_PAYLOAD_MASK, F64_EMPTY_NAN);
impl_NanPack!(f32, u32, F32_PAYLOAD_MASK, F32_EMPTY_NAN);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f64() {
        let f: f64 = NanPack::set_payload(0);
        assert!(f.is_nan());
        unsafe {
            let f_bits: u64 = transmute(f);
            let n_bits: u64 = transmute(::std::f64::NAN);
            assert_eq!(f_bits - 1, n_bits);
        }
        assert_eq!(f.get_payload(), Some(0));
    }

    #[test]
    fn various() {
        for i in 0..100 {
            let i = i * 4931;
            let f: f64 = NanPack::set_payload(i);
            assert!(f.is_nan());
            assert_eq!(f.get_payload(), Some(i));
        }
    }

    #[test]
    fn f32() {
        let f: f32 = NanPack::set_payload(0);
        assert!(f.is_nan());
        unsafe {
            let f_bits: u32 = transmute(f);
            let n_bits: u32 = transmute(::std::f32::NAN);
            assert_eq!(f_bits - 1, n_bits);
        }
        assert_eq!(f.get_payload(), Some(0));
    }

    #[test]
    #[should_panic]
    fn overflow_f32() {
        let _: f32 = NanPack::set_payload(0x20_0000 as usize);
    }

    #[test]
    #[should_panic]
    #[cfg(target_pointer_width = "64")]
    fn overflow_f64() {
        let _: f64 = NanPack::set_payload(::std::usize::MAX);
    }
}
