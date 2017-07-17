//! Applicative Functors in Rust

#[cfg(test)]
mod tests {
    use super::OptionApplicativeFunctor;

    #[test]
    fn it_applies_homomorphic_applicative_functor() {
        let af: Option<fn(u32) -> u32> = Some(|x: u32| x * 2);
        assert_eq!(af.ap(Some(21)), Some(42));
    }

    #[test]
    fn it_applies_non_homomorphic_applicative_functor() {
        let af: Option<fn(String) -> usize> = Some(|s: String| s.len());
        assert_eq!(af.ap(Some("test".to_string())), Some(4));
    }

    #[test]
    fn it_does_not_apply_from_none() {
        let naf: Option<fn(u32) -> u32> = None;
        assert_eq!(naf.ap(Some(42)), None);
    }

    #[test]
    fn it_does_not_apply_over_none() {
        let af: Option<fn(u32) -> u32> = Some(|x: u32| x * 2);
        assert_eq!(af.ap(None), None);
    }

    #[test]
    fn it_obeys_the_applicative_functor_rule() {
        let value = 21;
        assert_eq!(Some(|x: u32| x * 2).ap(Some(value)),
                   Some(value).map(|x: u32| x * 2));
    }
}


/// An OptionApplicativeFunctor trait.
///
/// This crate comes with default implementation of Applicative Functor for the Option type.
///
/// # Examples
///
/// ```
/// use self::applicative_functor::OptionApplicativeFunctor;
/// let af: Option<fn(u32) -> u32> = Some(|x: u32| x * 2);
/// assert_eq!(af.ap(Some(21)), Some(42));
/// ```
pub trait OptionApplicativeFunctor<F, T, U> {
    /// `ap` function always returns an Option.
    fn ap(self, other: Option<T>) -> Option<U>;
}

/// An OptionApplicativeFunctor implementation for Option that holds a function.
///
/// It obeys the applicative functor law:
/// `Some(f).ap(Some(a) == Some(a).map(f)`
impl<F, T, U> OptionApplicativeFunctor<F, T, U> for Option<F> where F: Fn(T) -> U {
    fn ap(self, other: Option<T>) -> Option<U> {
        match self {
            Some(f) => other.map(f),
            None => None,
        }
    }
}
