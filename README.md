# ov

The 'ov' crate provides a collection of traits that allow you to chain off of anything.
Each trait has a single method with the same name as the trait (but in snake case).

[`Over`](trait.Over.html), [`OverRef`](trait.OverRef.html), and [`OverMut`](trait.OverMut.html)
are each of `self`, `&self`, and `&mut self`, and the callback receives that same value.
They are implemented for all types.

[`OverDeref`](trait.OverDeref.html) and [`OverDerefMut`](trait.OverDerefMut.html) are implemented
for types which have `Deref` and `DerefMut` implementations. They both borrow the receiver,
and pass a reference of the `Deref::target` to the callback.

## Usage

You may either wildcard import it, or import specific traits. New items will not be added
in semver compatible versions.

```rust
use ov::*;
```
or
```rust
use ov::Over;
```

## Examples

A common use for this is with unary enum or struct constructors. These would otherwise
be multiple statements with variables, or nested parenthesis.

```rust
use ov::*;
use std::error::Error;
use std::io::{Error as IoError, ErrorKind};

fn maybe_string() -> Option<String> {
    "foo".to_owned().over(Some)
}
assert_eq!(maybe_string(), Some("foo".to_owned()));

// A helper is needed as `.over(Box::new)` always creates a Box<IoError>
fn to_err_object(error: impl Error + Send + 'static) -> Box<dyn Error + Send> {
    Box::new(error)
}

fn try_io_thing() -> Result<(), Box<dyn Error + Send>> {
    IoError::new(ErrorKind::Other, "oh no!").over(to_err_object).over(Err)
}
```

The `.over_mut` method can be used to perform some mutations on an arbitrary value,
including a field of a struct.

```rust
use ov::*;
struct S { field: i32 }
let mut s = S { field: 5 };

s.field.over_mut(|n| {
    // n is &mut i32
    *n *= 3;
    *n += 1;
});
assert_eq!(s.field, 16);
```

The `over_deref` and `over_deref_mut` methods can be useful if you want to use a function
that takes e.g. `&str` and you have a `String`.

```rust
let s = String::from("Hello, world!");
// Note: this would fail if `my_str` were `String` or `&String`
let len = s.over_deref(|my_str| str::len(my_str));
// Can be shortened to this
let len = s.over_deref(str::len);
assert_eq!(len, 13);
```

Another case for this is extracting the value from a mutable reference. For this, we
can use [`std::mem::take`] or [`std::mem::replace`]. In the following example we have
to use `replace` because `Command` doesn't impl `Default`.

[`std::mem::take`]: https://doc.rust-lang.org/stable/std/mem/fn.take.html
[`std::mem::replace`]: https://doc.rust-lang.org/stable/std/mem/fn.replace.html

```rust
use std::process::Command;
fn assert_type<T>(_x: T) {}

let command = Command::new("ls")
    .arg("-a") // returns &mut Command
    .arg("-l") // returns &mut Command
    .over(|r| std::mem::replace(r, Command::new(""))); // returns Command

assert_type::<Command>(command);

License: MIT
