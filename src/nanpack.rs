use std::mem::transmute;

const F32_PAYLOAD_MASK: u32 = 0x1F_FFFF;
const F32_EMPTY_NAN: u32 = 0x_7fc0_0000;

const F64_PAYLOAD_MASK: u64 = 0x3_FFFF_FFFF_FFFF;
const F64_EMPTY_NAN: u64 = 0x7ff8_0000_0000_0000;

pub trait NanPack<T> {
    fn set_payload(T) -> Self;
    fn get_payload(self) -> GetPayloadResult<T>;
}

#[derive(Debug, Eq, PartialEq)]
pub enum GetPayloadResult<T> {
    NotNan,
    EmptyNan,
    Some(T),
}

impl NanPack<usize> for f64 {
    fn set_payload(val: usize) -> Self {
        let val = (val + 1) as u64;
        assert!(val <= F64_PAYLOAD_MASK);
        unsafe { transmute(val | F64_EMPTY_NAN) }
    }

    fn get_payload(self) -> GetPayloadResult<usize> {
        if !self.is_nan() {
            return GetPayloadResult::NotNan;
        }
        let bits: u64 = unsafe { transmute(self) };
        let payload = bits & F64_PAYLOAD_MASK;
        if payload == 0 {
            return GetPayloadResult::EmptyNan;
        }
        let payload = payload - 1;
        if cfg!(target_pointer_width = "32") {
            assert!(payload <= ::std::u32::MAX as u64);
        }
        GetPayloadResult::Some(payload as usize)
    }
}

impl NanPack<usize> for f32 {
    fn set_payload(mut val: usize) -> Self {
        assert!(val < F32_PAYLOAD_MASK as usize);
        val += 1;
        unsafe { transmute(val as u32 | F32_EMPTY_NAN) }
    }

    fn get_payload(self) -> GetPayloadResult<usize> {
        if !self.is_nan() {
            return GetPayloadResult::NotNan;
        }
        let bits: u32 = unsafe { transmute(self) };
        let payload = bits & F32_PAYLOAD_MASK;
        if payload == 0 {
            return GetPayloadResult::EmptyNan;
        }
        let payload = payload - 1;
        GetPayloadResult::Some(payload as usize)
    }
}



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
        assert_eq!(f.get_payload(), GetPayloadResult::Some(0));
    }

    #[test]
    fn various() {
        for i in 0..100 {
            let i = i * 4931;
            let f: f64 = NanPack::set_payload(i);
            assert!(f.is_nan());
            assert_eq!(f.get_payload(), GetPayloadResult::Some(i));
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
        assert_eq!(f.get_payload(), GetPayloadResult::Some(0));
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
