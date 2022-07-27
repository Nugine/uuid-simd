#![allow(clippy::missing_safety_doc)]

#[derive(Debug, Clone)]
#[repr(C, align(16))]
pub struct Bytes16(pub [u8; 16]);

#[derive(Debug, Clone)]
#[repr(C, align(32))]
pub struct Bytes32(pub [u8; 32]);

pub unsafe trait Scalar: Copy {}

macro_rules! mark_scalar {
    ($($ty:ty),*) => {
        $(
            unsafe impl Scalar for $ty {}
        )*
    };
}

mark_scalar!(u8, u16, u32, u64, u128, usize);
mark_scalar!(i8, i16, i32, i64, i128, isize);
mark_scalar!(f32, f64);

#[inline(always)]
pub fn align16<T: Scalar>(slice: &[T]) -> (&[T], &[Bytes16], &[T]) {
    unsafe { slice.align_to() }
}

#[inline(always)]
pub fn align32<T: Scalar>(slice: &[T]) -> (&[T], &[Bytes32], &[T]) {
    unsafe { slice.align_to() }
}

#[inline(always)]
pub fn align<T: Scalar, U: Scalar>(slice: &[T]) -> (&[T], &[U], &[T]) {
    unsafe { slice.align_to() }
}
