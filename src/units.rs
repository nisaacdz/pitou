pub trait Unit {
    fn from_bytes(bytes: u64) -> Self;
    fn to_bytes(self) -> u64;
}

#[derive(Clone, Copy)]
pub struct Bytes(f64);


#[derive(Clone, Copy)]
pub struct Kilobytes(f64);


#[derive(Clone, Copy)]
pub struct Megabytes(f64);


#[derive(Clone, Copy)]
pub struct Gigabytes(f64);


macro_rules! impl_unit {
    ($type:ty, $val:literal) => {
        impl Unit for $type {
            fn from_bytes(bytes: u64) -> Self {
                let bytes = bytes as f64 / $val;
                Self(bytes)
            }

            fn to_bytes(self) -> u64 {
                (self.0 * $val) as _
            }
        }
    };
}

macro_rules! impl_val {
    ($type:ty) => {
        impl $type {
            pub fn val(&self) -> f64 {
                self.0
            }
        }
    };
}

macro_rules! impl_dbg {
    ($type:ty,$dp:literal) => {
        impl std::fmt::Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({:.prec$})", stringify!($type), self.0, prec = $dp) // Display two decimal places
            }
        }
    };
}

#[test]
fn test_me() {
    let dbg = stringify!(Debug);
    println!("{}", dbg);
    let dbg = Bytes::from_bytes(4);
    println!("{:?}", dbg);
}

impl_unit!(Bytes, 1f64);
impl_unit!(Kilobytes, 1024f64);
impl_unit!(Megabytes, 1048576f64);
impl_unit!(Gigabytes, 1073741824f64);

impl_dbg!(Bytes, 0);
impl_dbg!(Kilobytes, 2);
impl_dbg!(Megabytes, 2);
impl_dbg!(Gigabytes, 2);

impl_val!(Bytes);
impl_val!(Kilobytes);
impl_val!(Megabytes);
impl_val!(Gigabytes);