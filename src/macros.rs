macro_rules! declare {
    (
        $(#[$attribute:meta])*
        pub struct $native_type:ident => $foreign_type:ident,
        $($native_variant:ident => $foreign_variant:ident,)*
    ) => {
        $(#[$attribute])*
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $native_type {
            $($native_variant,)*
        }

        impl From<::ffi::$foreign_type> for $native_type {
            fn from(variant: ::ffi::$foreign_type) -> Self {
                match variant {
                    $(::ffi::$foreign_variant => $native_type::$native_variant,)*
                }
            }
        }
    }
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
    ($message:expr) => (return Err(::Error::from($message)));
);

macro_rules! success(
    ($status:expr) => (
        if let Some(error) = ::Error::current($status) {
            return Err(error);
        }
    );
);

macro_rules! ffi(
    ($function:ident($($argument:expr),*)) => (unsafe { ::ffi::$function($($argument),*) });
);
