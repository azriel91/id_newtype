//! Implements logic for a `Cow<'static, str>` newtype where only `[A-Za-z0-9_]`
//! are valid characters.
//!
//! Implementations are provided for:
//!
//! * `IdType::new`
//! * `IdType::new_unchecked` (with `#[doc(hidden)]`)
//! * `IdType::is_valid_id`
//! * `IdType::into_inner`
//! * `std::borrow::Borrow<str>`
//! * `std::convert::AsRef<str>`
//! * `std::convert::TryFrom<String>`
//! * `std::convert::TryFrom<&'static str>`
//! * `std::fmt::Display`
//! * `std::ops::Deref`
//! * `std::ops::DerefMut`
//! * `std::str::FromStr`
//!
//! A separate error type is also generated, which indicates an invalid value
//! when the ID type is instantiated with `new`.
//!
//!
//! # Usage
//!
//! ```rust
//! use std::borrow::Cow;
//!
//! use id_newtype::id_newtype;
//!
//! // Rename your ID type
//! #[derive(Clone, Debug, Hash, PartialEq, Eq)]
//! pub struct MyIdType(Cow<'static, str>);
//!
//! id_newtype!(
//!     MyIdType,           // Name of the ID type
//!     MyIdTypeInvalidFmt  // Name of the invalid value error
//! );
//! ```
//!
//! If you have a procedural macro that checks for ID validity<sup>1</sup> at
//! compile time, you may pass in its name as follows:
//!
//! ```rust,ignore
//! use std::borrow::Cow;
//!
//! use id_newtype::id_newtype;
//!
//! // replace this with your ID type's macro
//! use my_crate_static_check_macros::my_id_type;
//!
//! // Rename your ID type
//! #[derive(Clone, Debug, Hash, PartialEq, Eq)]
//! pub struct MyIdType(Cow<'static, str>);
//!
//! id_newtype!(
//!     MyIdType,           // Name of the ID type
//!     MyIdTypeInvalidFmt, // Name of the invalid value error
//!     my_id_type          // Name of the static check macro
//! );
//! ```
//!
//! <sup>1</sup> This crate was extracted from `peace`, so the
//! `my_crate_static_check_macros` is not generated for you. You must implement
//! it yourself. See [`static_check_macros`] for an example.
//!
//! [`static_check_macros`]: https://github.com/azriel91/peace/tree/0.0.14/crate/static_check_macros

#[macro_export]
macro_rules! id_newtype {
    ($ty_name:ident, $ty_err_name:ident) => {
        impl $ty_name {
            #[doc = concat!("Returns a new `", stringify!($ty_name), "` if the given `&str` is valid.")]
            ///
            #[doc = concat!("Most users should use the `", stringify!($macro_name), "!` macro as this provides")]
            /// compile time checks and returns a `const` value.
            pub fn new(s: &'static str) -> Result<Self, $ty_err_name<'static>> {
                Self::try_from(s)
            }

            #[doc = concat!("Returns a new `", stringify!($ty_name), "` without verification.")]
            ///
            #[doc = concat!("Most users should use the `", stringify!($macro_name), "!` macro as this provides")]
            /// compile time checks and returns a `const` value.
            ///
            /// This is here for guaranteed valid usage such as being called from the macro.
            #[doc(hidden)]
            pub const fn new_unchecked(s: &'static str) -> Self {
                Self(std::borrow::Cow::Borrowed(s))
            }
        }

        id_newtype!(IMPL; $ty_name, $ty_err_name);
    };

    ($ty_name:ident, $ty_err_name:ident, $macro_name:ident) => {
        impl $ty_name {
            #[doc = concat!("Returns a new `", stringify!($ty_name), "` if the given `&str` is valid.")]
            ///
            #[doc = concat!("Most users should use the `", stringify!($macro_name), "!` macro as this provides")]
            /// compile time checks and returns a `const` value.
            pub fn new(s: &'static str) -> Result<Self, $ty_err_name<'static>> {
                Self::try_from(s)
            }

            #[doc = concat!("Returns a new `", stringify!($ty_name), "` without verification.")]
            ///
            #[doc = concat!("Most users should use the `", stringify!($macro_name), "!` macro as this provides")]
            /// compile time checks and returns a `const` value.
            ///
            /// This is here for guaranteed valid usage such as being called from the macro.
            #[doc(hidden)]
            pub const fn new_unchecked(s: &'static str) -> Self {
                Self(std::borrow::Cow::Borrowed(s))
            }
        }

        id_newtype!(IMPL; $ty_name, $ty_err_name);
    };

    (IMPL; $ty_name:ident, $ty_err_name:ident) => {
        impl $ty_name {
            /// Returns whether the provided `&str` is a valid station identifier.
            pub fn is_valid_id(proposed_id: &str) -> bool {
                let mut chars = proposed_id.chars();
                let first_char = chars.next();
                let first_char_valid = first_char
                    .map(|c| c.is_ascii_alphabetic() || c == '_')
                    .unwrap_or(false);
                let remainder_chars_valid =
                    chars.all(|c| c.is_ascii_alphabetic() || c == '_' || c.is_ascii_digit());

                first_char_valid && remainder_chars_valid
            }

            /// Returns the inner `Cow<'static, str>`.
            pub fn into_inner(self) -> Cow<'static, str> {
                self.0
            }

            /// Returns the `&str` held by this ID.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl std::ops::Deref for $ty_name {
            type Target = std::borrow::Cow<'static, str>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $ty_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl TryFrom<String> for $ty_name {
            type Error = $ty_err_name<'static>;

            fn try_from(s: String) -> Result<$ty_name, $ty_err_name<'static>> {
                if Self::is_valid_id(&s) {
                    Ok($ty_name(std::borrow::Cow::Owned(s)))
                } else {
                    let s = std::borrow::Cow::Owned(s);
                    Err($ty_err_name::new(s))
                }
            }
        }

        impl TryFrom<&'static str> for $ty_name {
            type Error = $ty_err_name<'static>;

            fn try_from(s: &'static str) -> Result<$ty_name, $ty_err_name<'static>> {
                if Self::is_valid_id(s) {
                    Ok($ty_name(std::borrow::Cow::Borrowed(s)))
                } else {
                    let s = std::borrow::Cow::Borrowed(s);
                    Err($ty_err_name::new(s))
                }
            }
        }

        impl std::str::FromStr for $ty_name {
            type Err = $ty_err_name<'static>;

            fn from_str(s: &str) -> Result<$ty_name, $ty_err_name<'static>> {
                if Self::is_valid_id(s) {
                    Ok($ty_name(std::borrow::Cow::Owned(String::from(s))))
                } else {
                    let s = std::borrow::Cow::Owned(String::from(s));
                    Err($ty_err_name::new(s))
                }
            }
        }

        impl std::convert::AsRef<str> for $ty_name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl std::borrow::Borrow<str> for $ty_name {
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        impl<'s> std::borrow::Borrow<str> for &'s $ty_name {
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        #[doc = concat!("Error indicating `", stringify!($ty_name), "` provided is not in the correct format.")]
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $ty_err_name<'s> {
            /// String that was provided for the `$ty_name`.
            value: std::borrow::Cow<'s, str>,
        }

        impl<'s> $ty_err_name<'s> {
            #[doc = concat!("Returns a new `", stringify!($ty_err_name), "` error.")]
            pub fn new(value: std::borrow::Cow<'s, str>) -> Self {
                Self { value }
            }

            #[doc = concat!("Returns the value that failed to be parsed as a [`", stringify!($ty_name), "`].")]
            pub fn value(&self) -> &std::borrow::Cow<'s, str> {
                &self.value
            }
        }

        impl<'s> std::fmt::Display for $ty_err_name<'s> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "`{value}` is not a valid `{ty_name}`.\n\
                    `{ty_name}`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.",
                    ty_name = stringify!($ty_name),
                    value = self.value
                )
            }
        }

        impl<'s> std::error::Error for $ty_err_name<'s> {}
    };
}

#[cfg(test)]
mod tests {
    use std::borrow::{Borrow, Cow};

    #[derive(Clone, Debug, Hash, PartialEq, Eq)]
    pub struct MyIdType(Cow<'static, str>);

    crate::id_newtype!(
        MyIdType,           // Name of the ID type
        MyIdTypeInvalidFmt  // Name of the invalid value error
    );

    #[derive(Clone, Debug, Hash, PartialEq, Eq)]
    pub struct MyIdType2(Cow<'static, str>);

    crate::id_newtype!(
        MyIdType2,           // Name of the ID type
        MyIdType2InvalidFmt, // Name of the invalid value error
        my_id_static_macro
    );

    #[test]
    fn new() {
        let new_result = MyIdType::new("one");

        assert_eq!(Ok(MyIdType::new_unchecked("one")), new_result);
    }

    #[test]
    fn is_valid_id() {
        assert!(MyIdType::is_valid_id(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_"
        ));
        assert!(!MyIdType::is_valid_id("invalid with space"));
    }

    #[test]
    fn into_inner() {
        let my_id = MyIdType::new_unchecked("one");

        assert_eq!("one", my_id.into_inner());
    }

    #[test]
    fn as_str() {
        let my_id = MyIdType::new_unchecked("one");

        assert_eq!("one", my_id.as_str());
    }

    #[test]
    fn as_ref_str() {
        let my_id = MyIdType::new_unchecked("one");

        assert_eq!("one", my_id.as_ref());
    }

    #[test]
    fn borrow() {
        let my_id = MyIdType::new_unchecked("one");

        assert_eq!("one", Borrow::<str>::borrow(&my_id));
        assert_eq!("one", Borrow::<str>::borrow(&&my_id));
    }
}
