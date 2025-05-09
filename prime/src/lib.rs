pub fn prime_number() {
    for n in 2..=25 {
        let mut is_prime = true;

        for i in 2..=((n as f64).sqrt() as u64) {
            if n % i == 0 {
                is_prime = false;
                println!("{} is a product of {} * {}", n, i, n / i);
                break;
            }
        }

        if is_prime {
            println!("{} is prime.", n);
        }
    }
}

pub fn check_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

pub fn return_product_of_prime(n: u64) -> u64 {
    let mut product = 1;

    for i in 2..=n {
        if check_prime(i) {
            product *= i;
        }
    }

    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_number() {
        prime_number();
    }

    #[test]
    fn test_check_prime() {
        assert_eq!(check_prime(2), true);
        assert_eq!(check_prime(3), true);
        assert_eq!(check_prime(4), false);
    }

    #[test]
    fn test_product_of_prime() {
        assert_eq!(return_product_of_prime(5), 30); 
        assert_eq!(return_product_of_prime(10), 210); 
        assert_eq!(return_product_of_prime(1), 1); 
    }
}
