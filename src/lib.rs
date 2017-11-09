//! Applicative Functors in Rust

#[cfg(test)]
mod tests {
    mod option {
        use super::super::ApplicativeFunctor;
        
        #[test]
        fn it_applies_homomorphic_applicative_functor() {
            let af: Option<fn(u32) -> u32> = Some(|x: u32| x * 2);
            assert_eq!(af.ap(Some(21)), Some(42));
        }

        #[test]
        fn it_applies_non_homomorphic_applicative_functor() {
            let af: Option<fn(String) -> usize> = Some(|s: String| s.len());
            assert_eq!(af.ap(Some("test_option".to_string())), Some(11));
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
        fn it_obeys_the_applicative_functor_law() {
            let value = 21;
            let af: Option<fn(u32) -> u32> = Some(|x: u32| x * 2);
            assert_eq!(af.ap(Some(value)),
                    Some(value).map(|x: u32| x * 2));
        }
    }

    mod result {
        use super::super::ApplicativeFunctor;
        
        #[test]
        fn it_applies_homomorphic_applicative_functor() {
            let af: Result<fn(u32) -> u32, ()> = Ok(|x: u32| x * 3);
            assert_eq!(af.ap(Ok(22)), Ok(66));
        }

        #[test]
        fn it_applies_non_homomorphic_applicative_functor() {
            let af: Result<fn(String) -> usize, ()> = Ok(|s: String| s.len());
            assert_eq!(af.ap(Ok("test_result".to_string())), Ok(11));
        }

        #[test]
        fn it_does_not_apply_from_err() {
            let naf: Result<fn(u32) -> u32, ()> = Err(());
            assert_eq!(naf.ap(Ok(42)), Err(()));
        }

        #[test]
        fn it_does_not_apply_over_err() {
            let af: Result<fn(u32) -> u32, ()> = Ok(|x: u32| x * 3);
            assert_eq!(af.ap(Err(())), Err(()));
        }

        #[test]
        fn it_obeys_the_applicative_functor_law() {
            let value = 22;
            let af: Result<fn(u32) -> u32, ()> = Ok(|x: u32| x * 3);
            assert_eq!(af.ap(Ok(value)),
                Ok(value).map(|x: u32| x * 3));
        }
    }
}

/// An ApplicativeFunctor trait.
///
/// This crate comes with default implementation of Applicative Functor for the Option type.
///
/// # Examples
/// 
/// ```
/// use self::applicative_functor::ApplicativeFunctor;
/// 
/// let af_opt: Option<fn(u32) -> u32> = Some(|x: u32| x * 2);
/// assert_eq!(af_opt.ap(Some(21)), Some(42));
/// ```
/// ```
/// use self::applicative_functor::ApplicativeFunctor;
/// 
/// let af_res: Result<fn(f32) -> f32, ()> = Ok(|x: f32| x * 4.0);
/// assert_eq!(af_res.ap(Ok(10.5)), Ok(42.0));
/// ```
pub trait ApplicativeFunctor<F, T, U, A, B> {
    fn ap(self, other: A) -> B;
}

// impl<T> Mappable<T> for Option<T> {}

/// An ApplicativeFunctor implementation for `Option` that holds a function.
///
/// It obeys the applicative functor law:
/// `Some(f).ap(Some(a)) == Some(a).map(f)`
impl<F, T, U> ApplicativeFunctor<F, T, U, Option<T>, Option<U>> for Option<F> where F: Fn(T) -> U {
    fn ap(self, other: Option<T>) -> Option<U> {
        match self {
            Some(f) => other.map(f),
            None => None,
        }
    }
}

/// An ApplicativeFunctor implementation for `Result` that holds a function.
///
/// It obeys the applicative functor law:
/// `Ok(f).ap(Ok(a)) == Ok(a).map(f)`
impl<F, T, U, E> ApplicativeFunctor<F, T, U, Result<T, E>, Result<U, E>> for Result<F, E> where F: Fn(T) -> U {
    fn ap(self, other: Result<T, E>) -> Result<U, E> {
        match self {
            Ok(f) => other.map(f),
            Err(e) => Err(e),
        }
    }
}
