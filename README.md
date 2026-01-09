# 🪪 id_newtype

[![Crates.io](https://img.shields.io/crates/v/id_newtype.svg)](https://crates.io/crates/id_newtype)
[![docs.rs](https://img.shields.io/docsrs/id_newtype)](https://docs.rs/id_newtype)
[![CI](https://github.com/azriel91/id_newtype/workflows/CI/badge.svg)](https://github.com/azriel91/id_newtype/actions/workflows/ci.yml)
[![Coverage Status](https://codecov.io/gh/azriel91/id_newtype/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/id_newtype)

Implements logic for a `Cow<'static, str>` newtype where only `[A-Za-z0-9_]`
are valid characters.

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

A separate error type is also generated, which indicates an invalid value
when the ID type is instantiated with `new`.


# Usage

In `Cargo.toml`:

```toml
id_newtype = "0.3.0" # or
id_newtype = { version = "0.3.0", features = ["macros"] }
```

In code:

<details open>

```rust
// in lib.rs
#[macro_use]
extern crate id_newtype;

// in your ID module, e.g. `my_id.rs`
use std::borrow::Cow;

// Rename your ID type
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MyId(Cow<'static, str>);

id_newtype::id_newtype!(
    MyId,           // Name of the ID type
    MyIdInvalidFmt  // Name of the invalid value error
);
```

</details>

If you have a procedural macro that checks for ID validity<sup>1</sup> at
compile time, you may pass in its name as follows:

<details open>

```rust
#[macro_use]
extern crate id_newtype;

use std::borrow::Cow;

// Either use `id_newtype::id`, or replace this with your own proc macro.
use id_newtype::id;
// use my_crate_static_check_macros::my_id;

// Rename your ID type
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MyId(Cow<'static, str>);

id_newtype::id_newtype!(
    MyId,           // Name of the ID type
    MyIdInvalidFmt, // Name of the invalid value error
    my_id           // Name of the proc macro
);
```

<sup>1</sup> You can either enable the `"macros"` feature and have access to
the `id!` macro, or implement your own proc macro. See
[`id_newtype_macros`][macros_crate] for an example.

[macros_crate]: https://github.com/azriel91/id_newtype/id_newtype_macros

</details>

Finally, if you pass in a lifetime parameter (4th param) to the macro, the
generated type can hold borrowed data:

<details open>

```rust
#[macro_use]
extern crate id_newtype;

use std::borrow::Cow;

// Either use `id_newtype::id`, or replace this with your own proc macro.
use id_newtype::id;
// use my_crate_static_check_macros::my_id;

// Rename your ID type
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MyId<'id>(Cow<'id, str>);

id_newtype::id_newtype!(
    MyId,           // Name of the ID type
    MyIdInvalidFmt, // Name of the invalid value error
    my_id           // Name of the proc macro
    'id             // Lifetime parameter
);
```

A `MyId<'id>` can be converted into `MyId<'static>` by using
`my_id.into_static()` which takes `self`.

</details>


## Features

* `"macros"` This feature enables the `id!` compile-time checked proc macro
  for safe construction of IDs at compile time.

    ```rust
    # #[cfg(feature = "my_feature")]
    # {
    #[macro_use]
    extern crate id_newtype;
    use id_newtype::id;

    // Define a new ID type
    #[derive(Clone, Debug, Hash, PartialEq, Eq)]
    pub struct MyId(Cow<'static, str>);
    id_newtype::id_newtype!(MyId, MyIdInvalidFmt, id);

    // ok!
    let id = id!("my_id");

    // `id` is not a valid `Id`
    // `Id`s must begin with a letter or underscore, and contain only
    // letters, numbers, or underscores.
    let id = id!("invalid id");
    # }
    ```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE] or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT] or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
