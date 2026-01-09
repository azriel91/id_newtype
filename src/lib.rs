//! Implements logic for a `Cow<'static, str>` newtype where only `[A-Za-z0-9_]`
//! are valid characters.
//!
//! Implementations are provided for:
//!
//! * `IdType::new`
//! * `IdType::new_unchecked` (with `#[doc(hidden)]`)
//! * `IdType::is_valid_id`
//! * `IdType::into_inner`
//! * `IdType::into_static`
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
//! In `Cargo.toml`:
//!
//! ```toml
//! id_newtype = "0.3.0" # or
//! id_newtype = { version = "0.3.0", features = ["macros"] }
//! ```
//!
//! In code:
//!
//! ```rust
//! // in lib.rs
//! #[macro_use]
//! extern crate id_newtype;
//!
//! // in your ID module, e.g. `my_id.rs`
//! use std::borrow::Cow;
//!
//! // Rename your ID type
//! #[derive(Clone, Debug, Hash, PartialEq, Eq)]
//! pub struct MyId(Cow<'static, str>);
//!
//! id_newtype::id_newtype!(
//!     MyId,           // Name of the ID type
//!     MyIdInvalidFmt  // Name of the invalid value error
//! );
//! ```
//!
//! If you have a procedural macro that checks for ID validity<sup>1</sup> at
//! compile time, you may pass in its name as follows:
//!
//! ```rust
//! #[macro_use]
//! extern crate id_newtype;
//!
//! use std::borrow::Cow;
//!
//! // Either use `id_newtype::id`, or replace this with your own proc macro.
//! use id_newtype::id;
//! // use my_crate_static_check_macros::my_id;
//!
//! // Rename your ID type
//! #[derive(Clone, Debug, Hash, PartialEq, Eq)]
//! pub struct MyId(Cow<'static, str>);
//!
//! id_newtype::id_newtype!(
//!     MyId,           // Name of the ID type
//!     MyIdInvalidFmt, // Name of the invalid value error
//!     my_id           // Name of the proc macro
//! );
//! ```
//!
//! <sup>1</sup> You can either enable the `"macros"` feature and have access to
//! the `id!` macro, or implement your own proc macro. See
//! [`id_newtype_macros`][macros_crate] for an example.
//!
//! [macros_crate]: https://github.com/azriel91/id_newtype/id_newtype_macros
//!
//! ## Lifetime-Parameterized ID Types
//!
//! If you want your ID type to support borrowed strings with non-`'static`
//! lifetimes, you can pass a lifetime parameter as the fourth argument:
//!
//! ```rust
//! #[macro_use]
//! extern crate id_newtype;
//!
//! use std::borrow::Cow;
//!
//! // Define an ID type with a generic lifetime
//! #[derive(Clone, Debug, Hash, PartialEq, Eq)]
//! pub struct MyId<'s>(Cow<'s, str>);
//!
//! id_newtype::id_newtype!(
//!     MyId,           // Name of the ID type
//!     MyIdInvalidFmt, // Name of the invalid value error
//!     my_id,          // Name of the proc macro
//!     's              // Lifetime parameter
//! );
//! ```
//!
//! ## Features
//!
//! * `"macros"` This feature enables the `id!` compile-time checked proc macro
//!   for safe construction of IDs at compile time.
//!
//!     ```rust
//!     # #[cfg(feature = "my_feature")]
//!     # {
//!     #[macro_use]
//!     extern crate id_newtype;
//!     use id_newtype::id;
//!
//!     // Define a new ID type
//!     #[derive(Clone, Debug, Hash, PartialEq, Eq)]
//!     pub struct MyId(Cow<'static, str>);
//!     id_newtype::id_newtype!(MyId, MyIdInvalidFmt, id);
//!
//!     // ok!
//!     let id = id!("my_id");
//!
//!     // `id` is not a valid `Id`
//!     // `Id`s must begin with a letter or underscore, and contain only
//!     // letters, numbers, or underscores.
//!     let id = id!("invalid id");
//!     # }
//!     ```

// Re-export the compiled-time checked constructor.
#[cfg(feature = "macros")]
pub use id_newtype_macros::id;

#[macro_export]
macro_rules! id_newtype {
    // No macro name, no lifetime
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

    // With macro name, no lifetime
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

    // With macro name and lifetime parameter (new)
    ($ty_name:ident, $ty_err_name:ident, $macro_name:ident, $lt:lifetime) => {
        impl<$lt> $ty_name<$lt> {
            #[doc = concat!("Returns a new `", stringify!($ty_name), "` if the given `&str` is valid.")]
            ///
            #[doc = concat!("Most users should use the `", stringify!($macro_name), "!` macro as this provides")]
            /// compile time checks and returns a `const` value.
            pub fn new(s: &$lt str) -> Result<Self, $ty_err_name<$lt>> {
                Self::try_from(s)
            }

            #[doc = concat!("Returns a new `", stringify!($ty_name), "` without verification.")]
            ///
            #[doc = concat!("Most users should use the `", stringify!($macro_name), "!` macro as this provides")]
            /// compile time checks and returns a `const` value.
            ///
            /// This is here for guaranteed valid usage such as being called from the macro.
            #[doc(hidden)]
            pub const fn new_unchecked(s: &$lt str) -> $ty_name<$lt> {
                $ty_name(std::borrow::Cow::Borrowed(s))
            }
        }

        id_newtype!(IMPL_LT; $ty_name, $ty_err_name, $lt);
    };

    // Implementation for static lifetime types
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

    // Implementation for lifetime-parameterized types (new)
    (IMPL_LT; $ty_name:ident, $ty_err_name:ident, $lt:lifetime) => {
        impl<$lt> $ty_name<$lt> {
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

            #[doc = concat!("Returns the inner `Cow<'", stringify!($lt), ", str>`.")]
            pub fn into_inner(self) -> Cow<$lt, str> {
                self.0
            }

            #[doc = concat!("Returns this with owned data.")]
            pub fn into_static(self) -> $ty_name<'static> {
                $ty_name(Cow::Owned(self.0.into_owned()))
            }

            /// Returns the `&str` held by this ID.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl<$lt> std::ops::Deref for $ty_name<$lt> {
            type Target = std::borrow::Cow<$lt, str>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$lt> std::fmt::Display for $ty_name<$lt> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl<$lt> TryFrom<String> for $ty_name<$lt> {
            type Error = $ty_err_name<'static>;

            fn try_from(s: String) -> Result<$ty_name<$lt>, $ty_err_name<'static>> {
                if Self::is_valid_id(&s) {
                    Ok($ty_name(std::borrow::Cow::Owned(s)))
                } else {
                    let s = std::borrow::Cow::Owned(s);
                    Err($ty_err_name::new(s))
                }
            }
        }

        impl<$lt> TryFrom<&$lt str> for $ty_name<$lt> {
            type Error = $ty_err_name<$lt>;

            fn try_from(s: &$lt str) -> Result<$ty_name<$lt>, $ty_err_name<$lt>> {
                if Self::is_valid_id(s) {
                    Ok($ty_name(std::borrow::Cow::Borrowed(s)))
                } else {
                    let s = std::borrow::Cow::Borrowed(s);
                    Err($ty_err_name::new(s))
                }
            }
        }

        impl<$lt> std::str::FromStr for $ty_name<$lt> {
            type Err = $ty_err_name<'static>;

            fn from_str(s: &str) -> Result<$ty_name<$lt>, $ty_err_name<'static>> {
                if Self::is_valid_id(s) {
                    Ok($ty_name(std::borrow::Cow::Owned(String::from(s))))
                } else {
                    let s = std::borrow::Cow::Owned(String::from(s));
                    Err($ty_err_name::new(s))
                }
            }
        }

        impl<$lt> std::convert::AsRef<str> for $ty_name<$lt> {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl<$lt> std::borrow::Borrow<str> for $ty_name<$lt> {
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        impl<'__borrow, $lt> std::borrow::Borrow<str> for &'__borrow $ty_name<$lt> {
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        #[doc = concat!("Error indicating `", stringify!($ty_name), "` provided is not in the correct format.")]
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $ty_err_name<'__err> {
            /// String that was provided for the `$ty_name`.
            value: std::borrow::Cow<'__err, str>,
        }

        impl<'__err> $ty_err_name<'__err> {
            #[doc = concat!("Returns a new `", stringify!($ty_err_name), "` error.")]
            pub fn new(value: std::borrow::Cow<'__err, str>) -> Self {
                Self { value }
            }

            #[doc = concat!("Returns the value that failed to be parsed as a [`", stringify!($ty_name), "`].")]
            pub fn value(&self) -> &std::borrow::Cow<'__err, str> {
                &self.value
            }
        }

        impl<'__err> std::fmt::Display for $ty_err_name<'__err> {
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

        impl<'__err> std::error::Error for $ty_err_name<'__err> {}
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

    // Test for lifetime-parameterized ID type
    #[derive(Clone, Debug, Hash, PartialEq, Eq)]
    pub struct MyIdType3<'s>(Cow<'s, str>);

    crate::id_newtype!(
        MyIdType3,           // Name of the ID type
        MyIdType3InvalidFmt, // Name of the invalid value error
        my_id_lt_macro,      // Name of the proc macro
        's                   // Lifetime parameter
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

    // Tests for lifetime-parameterized ID type
    #[test]
    fn lt_new() {
        let new_result = MyIdType3::new("one");

        assert_eq!(Ok(MyIdType3::new_unchecked("one")), new_result);
    }

    #[test]
    fn lt_new_with_borrowed_string() {
        let s = String::from("valid_id");
        let new_result: Result<MyIdType3<'_>, _> = MyIdType3::try_from(s.as_str());

        assert!(new_result.is_ok());
        assert_eq!("valid_id", new_result.unwrap().as_str());
    }

    #[test]
    fn lt_new_invalid() {
        let s = String::from("invalid id");
        let new_result: Result<MyIdType3<'_>, _> = MyIdType3::try_from(s.as_str());

        assert!(new_result.is_err());
    }

    #[test]
    fn lt_is_valid_id() {
        assert!(MyIdType3::is_valid_id(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_"
        ));
        assert!(!MyIdType3::is_valid_id("invalid with space"));
    }

    #[test]
    fn lt_into_inner() {
        let my_id = MyIdType3::new_unchecked("one");

        assert_eq!("one", my_id.into_inner());
    }

    #[test]
    fn lt_into_static() {
        let s = String::from("one");
        let my_id = MyIdType3::new_unchecked(s.as_str());
        let my_id_static = my_id.into_static();

        assert_eq!("one", my_id_static.as_ref());

        drop(s);

        // If this compiles, it means `my_id_static` is not tied to the lifetime of `s`.
        drop(my_id_static);
    }

    #[test]
    fn lt_as_str() {
        let my_id = MyIdType3::new_unchecked("one");

        assert_eq!("one", my_id.as_str());
    }

    #[test]
    fn lt_as_ref_str() {
        let my_id = MyIdType3::new_unchecked("one");

        assert_eq!("one", my_id.as_ref());
    }

    #[test]
    fn lt_borrow() {
        let my_id = MyIdType3::new_unchecked("one");

        assert_eq!("one", Borrow::<str>::borrow(&my_id));
        assert_eq!("one", Borrow::<str>::borrow(&&my_id));
    }

    #[test]
    fn lt_from_str() {
        use std::str::FromStr;

        let my_id: MyIdType3<'_> = MyIdType3::from_str("valid_id").unwrap();
        assert_eq!("valid_id", my_id.as_str());
    }

    #[test]
    fn lt_try_from_string() {
        let my_id: MyIdType3<'_> = MyIdType3::try_from(String::from("valid_id")).unwrap();
        assert_eq!("valid_id", my_id.as_str());
    }

    #[test]
    fn lt_display() {
        let my_id = MyIdType3::new_unchecked("one");
        assert_eq!("one", format!("{}", my_id));
    }

    #[test]
    fn lt_deref() {
        let my_id = MyIdType3::new_unchecked("one");
        let cow: &Cow<'_, str> = &*my_id;
        assert_eq!("one", cow.as_ref());
    }

    #[test]
    fn lt_borrowed_lifetime_tied_to_source() {
        // This test verifies that the ID's lifetime is properly tied to the source
        // string. The ID borrows from `s` and its lifetime is bounded by `s`.
        fn create_id_from_str<'a>(s: &'a str) -> Result<MyIdType3<'a>, MyIdType3InvalidFmt<'a>> {
            MyIdType3::try_from(s)
        }

        let owned = String::from("test_id");
        let id = create_id_from_str(&owned).unwrap();
        assert_eq!("test_id", id.as_str());

        // Verify that owned strings still work via FromStr
        let owned_id: MyIdType3<'_> = owned.parse().unwrap();
        assert_eq!("test_id", owned_id.as_str());
    }
}
