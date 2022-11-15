use crate::vector::{V128, V256, V512, V64};

pub unsafe trait POD: Copy + 'static {
    const ID: PodTypeId;
}

macro_rules! mark_pod {
    ($($ty:ident),*) => {
        $(
            unsafe impl POD for $ty {
                const ID: PodTypeId = PodTypeId::$ty;
            }
        )*
    };
}

mark_pod!(u8, u16, u32, u64, u128, usize);
mark_pod!(i8, i16, i32, i64, i128, isize);
mark_pod!(f32, f64);
mark_pod!(V64, V128, V256, V512);

#[inline(always)]
pub fn align<T: POD, U: POD>(slice: &[T]) -> (&[T], &[U], &[T]) {
    unsafe { slice.align_to() }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum PodTypeId {
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,

    i8,
    i16,
    i32,
    i64,
    i128,
    isize,

    f32,
    f64,

    V64,
    V128,
    V256,
    V512,
}

#[macro_export]
macro_rules! is_pod_type {
    ($self:ident, $x:ident $(| $xs:ident)*) => {{
        // TODO: inline const
        matches!(<$self as $crate::pod::POD>::ID, $crate::pod::PodTypeId::$x $(| $crate::pod::PodTypeId::$xs)*)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    const _: () = {
        const fn is_8bits<T: POD>() -> bool {
            is_pod_type!(T, u8 | i8)
        }

        assert!(is_8bits::<u8>());
        assert!(is_8bits::<i8>());
    };
}
