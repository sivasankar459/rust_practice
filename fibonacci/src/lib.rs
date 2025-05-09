
pub fn fibonacci(n: u64) -> Vec<u64> {
   if n == 0 {
        return vec![];
    } else if n == 1 {
        return vec![0];
    }

    let mut fib_series = vec![0, 1];

    for i in 2..n {
        // fib_series[i] = fib_series[i - 1] + fib_series[i - 2];
        let next_fib = fib_series[(i - 1) as usize] + fib_series[(i - 2) as usize];
        fib_series.push(next_fib);
    }
    
    fib_series
    
}


fn fib(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2); // Recursive case: F(n) = F(n-1) + F(n-2)
    }
}

pub fn fibonacci_recursive(n: u64) -> Vec<u64> {

    let mut series = Vec::with_capacity(n as usize);

    for i in 0..n {
        series.push(fib(i)); 
    }

    series
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_series() {

        let result = fibonacci(1);
        assert_eq!(result, vec![0]);

        let result = fibonacci(2);
        assert_eq!(result, vec![0, 1]);

        let result = fibonacci(3);
        assert_eq!(result, vec![0, 1, 1]);
        
    }
    #[test]
    fn test_fibonacci_recursive() {

        let result  = fibonacci_recursive(1);
        assert_eq!(result, vec![0]);

        let result  = fibonacci_recursive(2);
        assert_eq!(result, vec![0, 1]);

        let result = fibonacci_recursive(3);
        assert_eq!(result, vec![0, 1, 1]);

    }
}
