//! Applicative Functors in Rust

#[cfg(test)]
mod tests {
    mod option {
        use super::super::Applicative;
        
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
        use super::super::Applicative;
        
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


/// Functor trait.
///
/// This crate comes with default implementation of Functor for the `Option` and `Result` types.
pub trait Functor<T> {
    fn map<F>(self, f: F) -> Self where F: Fn(T) -> T;
}

impl<T> Functor<T> for Option<T> {
    fn map<F>(self, f: F) -> Self where F: Fn(T) -> T {
        self.map(f)
    }
}

impl<T, E> Functor<T> for Result<T, E> {
    fn map<F>(self, f: F) -> Self where F: Fn(T) -> T {
        self.map(f)
    }
}

/// An Applicative trait.
///
/// This crate comes with default implementation of Applicative for the `Option` and `Result` types.
///
/// # Examples
/// 
/// ```
/// use self::applicative_functor::Applicative;
/// 
/// let af_opt: Option<fn(u32) -> u32> = Some(|x: u32| x * 2);
/// assert_eq!(af_opt.ap(Some(21)), Some(42));
/// ```
/// ```
/// use self::applicative_functor::Applicative;
/// 
/// let af_res: Result<fn(f32) -> f32, ()> = Ok(|x: f32| x * 4.0);
/// assert_eq!(af_res.ap(Ok(10.5)), Ok(42.0));
/// ```
pub trait Applicative<T, U, A: Functor<T>, B: Functor<U>> {
    fn ap(self, other: A) -> B;
}

impl<F, T, U> Applicative<T, U, Option<T>, Option<U>> for Option<F> where F: Fn(T) -> U {
    fn ap(self, other: Option<T>) -> Option<U> {
        match self {
            Some(f) => other.map(f),
            None => None,
        }
    }
}

impl<F, T, U, E> Applicative<T, U, Result<T, E>, Result<U, E>> for Result<F, E> where F: Fn(T) -> U {
    fn ap(self, other: Result<T, E>) -> Result<U, E> {
        match self {
            Ok(f) => other.map(f),
            Err(e) => Err(e),
        }
    }
}
