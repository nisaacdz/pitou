pub trait Unit {
    fn from_bytes(bytes: u64) -> Self;
}

pub struct Bytes(u64);
pub struct Kilobytes(u64);
pub struct Megabytes(u64);
pub struct Gigabytes(u64);

macro_rules! impl_unit {
    ($type:ty, $val:literal) => {
        impl Unit for $type {
            fn from_bytes(bytes: u64) -> Self {
                Self(bytes / (1 << $val))
            }
        }
    };
}

impl_unit!(Bytes, 0);
impl_unit!(Kilobytes, 10);
impl_unit!(Megabytes, 20);
impl_unit!(Gigabytes, 30);