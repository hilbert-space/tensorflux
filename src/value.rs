use ffi::TF_DataType;

/// A value.
pub trait Value: 'static {
    #[doc(hidden)]
    fn kind() -> TF_DataType;
}

macro_rules! implement {
    ($($native:path => $variant:ident,)*) => {
        $(impl Value for $native {
            #[inline(always)]
            fn kind() -> TF_DataType {
                TF_DataType::$variant
            }
        })*
    }
}

implement! {
    bool => TF_BOOL,
    f32 => TF_FLOAT,
    f64 => TF_DOUBLE,
    i8 => TF_INT8,
    i16 => TF_INT16,
    i32 => TF_INT32,
    i64 => TF_INT64,
    u8 => TF_UINT8,
    u16 => TF_UINT16,
    String => TF_STRING,
}

#[cfg(feature = "complex")]
implement! {
    ::c32 => TF_COMPLEX64,
    ::c64 => TF_COMPLEX128,
}
