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
            if let Some(error) = ::result::Error::current($status) {
                return Err(error);
            }
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
    ($result:expr) => (match $result {
        Ok(result) => result,
        Err(error) => raise!(format!("{}", error)),
    });
);

macro_rules! raise(
    ($message:expr) => (return Err(::result::Error::from($message)));
);
