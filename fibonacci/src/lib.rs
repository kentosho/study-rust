pub mod fibonacci;

// Tests
#[cfg(test)]
mod tests_fibonacci {
    use crate::fibonacci::*;
    #[test]
    fn test_fib_match() {
        assert_eq!(55, fib_match(10));
    }
    #[test]
    fn test_fib_one() {
        assert_eq!(55, fib_one(10));
    }
    #[test]
    fn test_fib_dp() {
        assert_eq!(55, fib_dp_simple(10));
    }
}
