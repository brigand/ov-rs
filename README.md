# ov

The 'ov' crate provides a collection of traits that allow you to chain off of anything.
Each trait has a single method with the same name as the trait (but in snake case).

[`Over`](trait.Over.html), [`OverRef`](trait.OverRef.html), and [`OverMut`](trait.OverMut.html)
are each of `self`, `&self`, and `&mut self`, and the callback receives that same value.
They are implemented for all types.

[`OverDeref`](trait.OverDeref.html) and [`OverDerefMut`](trait.OverDerefMut.html) are implemented
for types which have `Deref` and `DerefMut` implementations. They both borrow the receiver,
and pass a reference of the `Deref::target` to the callback.

## Examples

```rust
use ov::*;
let mut n = 5;
assert_eq!(n.over(|n| n * 2), 10);
n.over_mut(|n| {
  *n *= 3
});
assert_eq!(n, 15);

let s = String::from("Hello, world!");
// Note: this would fail if `s` is `String` or `&String`
let len = s.over_deref(|s| str::len(s));
assert_eq!(len, 13);
```
