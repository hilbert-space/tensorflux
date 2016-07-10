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

macro_rules! memory {
    ($kind:ident<T>) => (
        deref!($kind::memory<T>);

        impl<T> ::std::convert::AsRef<[T]> for $kind<T> {
            #[inline]
            fn as_ref(&self) -> &[T] {
                &self.memory
            }
        }

        impl<T> Into<Vec<T>> for $kind<T> where T: Clone {
            #[inline]
            fn into(mut self) -> Vec<T> {
                self.memory.empty()
            }
        }
    );
    ($kind:ident<$element:ident>) => (
        deref!($kind::memory<$element>);

        impl ::std::convert::AsRef<[$element]> for $kind {
            #[inline]
            fn as_ref(&self) -> &[$element] {
                &self.memory
            }
        }

        impl Into<Vec<$element>> for $kind {
            #[inline]
            fn into(mut self) -> Vec<$element> {
                self.memory.empty()
            }
        }
    );
}

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
    ($message:expr) => (return Err(::error::Error::from($message)));
);

macro_rules! success(
    ($status:expr) => (
        if let Some(error) = ::error::Error::from_status($status) {
            return Err(error);
        }
    );
);
