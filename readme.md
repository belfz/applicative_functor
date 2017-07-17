## applicative_functor in Rust

The point of this crate is to add applicative functor's functionality to existing monadic entities in Rust's standard library.

Currently, an implementation for `Option` is available, adding the `ap(self, other: Option<T>) -> Option<U>` method to `Option`'s interface.

### Example

```Rust
extern crate applicative_functor;
use applicative_functor::OptionApplicativeFunctor;

let af: Option<fn(u32) -> u32> = Some(|x: u32| x * 2);
assert_eq!(af.ap(Some(21)), Some(42));
```

__Note__: this crate has not been published to crates.io yet.