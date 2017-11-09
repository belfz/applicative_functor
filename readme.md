## applicative_functor in Rust

The point of this crate is to add applicative functor's interface to existing monadic entities in Rust's standard library: `Option<T>` and `Result<T, E>`.

### Example

```rust
extern crate applicative_functor;
use applicative_functor::ApplicativeFunctor;

let af_opt: Option<fn(u32) -> u32> = Some(|x: u32| x * 2);
assert_eq!(af_opt.ap(Some(21)), Some(42));

let af_res: Result<fn(f32) -> f32, ()> = Ok(|x: f32| x * 4.0);
assert_eq!(af_res.ap(Ok(10.5)), Ok(42.0));
```

__Note__: this crate has not been published to crates.io yet.