pub fn fahrenheit_to_celsius(celsius: f64) -> f64 {
    // Convert Fahrenheit to Celsius Formula: (F - 32) * 5/9
    let celsius = (celsius - 32.0) * 5.0 / 9.0;
    // Round to two decimal places
    (celsius * 10.0).round() / 10.0 
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_fahrenheit_to_celsius() {
        
        let result = fahrenheit_to_celsius(32.0);
        assert_eq!(result, 0.0);

        let result = fahrenheit_to_celsius(212.0);
        assert_eq!(result, 100.0);

        let result = fahrenheit_to_celsius(98.6);
        assert_eq!(result, 37.0);

        let result = fahrenheit_to_celsius(0.0);
        assert_eq!(result, -17.8);

        let result = fahrenheit_to_celsius(-1.4);
        assert_eq!(result, -18.6);
    }
}
