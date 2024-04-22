pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

/*Formule
F-C (32 °F − 32) × 5/9 = 0 °C
C-F (32 °C × 9/5) + 32 = 89,6 °F
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fahrenheit_to_celsius(-459.67);
        assert_eq!(result, -273.15);
        let result = celsius_to_fahrenheit(0.0);
        assert_eq!(result, 32.0);

    }
}
