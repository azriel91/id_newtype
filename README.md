# 🪪 id_newtype

[![Crates.io](https://img.shields.io/crates/v/id_newtype.svg)](https://crates.io/crates/id_newtype)
[![docs.rs](https://img.shields.io/docsrs/id_newtype)](https://docs.rs/id_newtype)
[![CI](https://github.com/azriel91/id_newtype/workflows/CI/badge.svg)](https://github.com/azriel91/id_newtype/actions/workflows/ci.yml)
[![Coverage Status](https://codecov.io/gh/azriel91/id_newtype/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/id_newtype)

Implements logic for a `Cow<'static, str>` newtype where only `[A-Za-z0-9_]` are valid characters.

Implementations are provided for:

* `IdType::new`
* `IdType::new_unchecked` (with `#[doc(hidden)]`)
* `IdType::is_valid_id`
* `IdType::into_inner`
* `std::borrow::Borrow<str>`
* `std::convert::AsRef<str>`
* `std::convert::TryFrom<String>`
* `std::convert::TryFrom<&'static str>`
* `std::fmt::Display`
* `std::ops::Deref`
* `std::ops::DerefMut`
* `std::str::FromStr`

A separate error type is also generated, which indicates an invalid value when the ID type is instantiated with `new`.


# Usage

```rust
use std::borrow::Cow;

// Rename your ID type
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MyIdType(Cow<'static, str>);

crate::id_newtype!(
    MyIdType,           // Name of the ID type
    MyIdTypeInvalidFmt  // Name of the invalid value error
);
```

If you have a procedural macro that checks for ID validity<sup>1</sup> at compile time, you may pass in its name as follows:

```rust
use std::borrow::Cow;

// replace this with your ID type's macro
use my_crate_static_check_macros::my_id_type;

// Rename your ID type
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MyIdType(Cow<'static, str>);

crate::id_newtype!(
    MyIdType,           // Name of the ID type
    MyIdTypeInvalidFmt, // Name of the invalid value error
    my_id_type          // Name of the static check macro
);
```

<sup>1</sup> This crate was extracted from `peace`, so the `my_crate_static_check_macros` is not generated for you. You must implement it yourself. See [`static_check_macros`] for an example.

[`static_check_macros`]: https://github.com/azriel91/peace/tree/0.0.14/crate/static_check_macros


## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE] or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT] or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
