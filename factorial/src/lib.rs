pub fn factorial(a: u64) -> u64 {

    let mut result = 1;
    for i in 1..=a {
        result *= i;
    }

    result
}

pub fn factorial_recursive(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial_recursive(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_calculate_factorial() {
        
        let result = factorial(5);
        assert_eq!(result, 120);

        let result = factorial(0);
        assert_eq!(result, 1);

        let result = factorial(1);
        assert_eq!(result, 1);

        let result = factorial(10);
        assert_eq!(result, 3628800);

    }

    #[test]
    fn test_to_calculate_factorial_recursive() {
        
        let result = factorial_recursive(5);
        assert_eq!(result, 120);

        let result = factorial_recursive(0);
        assert_eq!(result, 1);

        let result = factorial_recursive(1);
        assert_eq!(result, 1);

        let result = factorial_recursive(10);
        assert_eq!(result, 3628800);

    }
}
