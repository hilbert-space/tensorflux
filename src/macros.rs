macro_rules! deref {
    ($kind:ident::$field:ident<T>) => (
        impl<T> ::std::ops::Deref for $kind<T> {
            type Target = [T];

            #[inline]
            fn deref(&self) -> &[T] {
                &self.$field
            }
        }

        impl<T> ::std::ops::DerefMut for $kind<T> {
            #[inline]
            fn deref_mut(&mut self) -> &mut [T] {
                &mut self.$field
            }
        }
    );
    ($kind:ident::$field:ident<$element:ident>) => (
        impl ::std::ops::Deref for $kind {
            type Target = [$element];

            #[inline]
            fn deref(&self) -> &[$element] {
                &self.$field
            }
        }

        impl ::std::ops::DerefMut for $kind {
            #[inline]
            fn deref_mut(&mut self) -> &mut [$element] {
                &mut self.$field
            }
        }
    );
}

macro_rules! ffi(
    ($function:ident($($argument:expr),*)) => (unsafe { ::ffi::$function($($argument),*) });
);

macro_rules! into_cstring(
    ($string:expr) => (unsafe { ::std::ffi::CString::from_vec_unchecked($string.into().into()) });
);

macro_rules! nonnull(
    ($pointer:expr, $status:expr) => ({
        let pointer = $pointer;
        if pointer.is_null() {
            success!($status);
            raise!("failed to call TensorFlow");
        }
        pointer
    });
    ($pointer:expr) => ({
        let pointer = $pointer;
        if pointer.is_null() {
            raise!("failed to call TensorFlow");
        }
        pointer
    });
);

macro_rules! ok(
    ($operation:expr, $status:expr) => ({
        let result = $operation;
        success!($status);
        result
    });
    ($result:expr) => (match $result {
        Ok(result) => result,
        Err(error) => raise!(error.to_string()),
    });
);

macro_rules! raise(
    ($template:expr, $($argument:tt)*) => (raise!(format!($template, $($argument)*)));
    ($message:expr) => (return Err(::Error::from($message)));
);

macro_rules! success(
    ($status:expr) => (
        if let Some(error) = ::Error::current($status) {
            return Err(error);
        }
    );
);

macro_rules! translate {
    (
        $(#[$attribute:meta])*
        pub struct $from_type:ident => $into_type:ident,
        $($from_variant:ident => $into_variant:ident,)*
    ) => {
        $(#[$attribute])*
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $from_type {
            $($from_variant,)*
        }

        impl From<::ffi::$into_type> for $from_type {
            fn from(variant: ::ffi::$into_type) -> Self {
                match variant {
                    $(::ffi::$into_variant => $from_type::$from_variant,)*
                }
            }
        }

        impl From<$from_type> for ::ffi::$into_type {
            fn from(variant: $from_type) -> Self {
                match variant {
                    $($from_type::$from_variant => ::ffi::$into_variant,)*
                }
            }
        }
    }
}

