use case::CaseExt;
use edit_distance::edit_distance;

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    let compared_normalized = compared.to_lowercase();
    let expected_normalized = expected.to_lowercase();

    if (!compared_normalized.is_camel_lowercase()
        && compared.to_camel().to_snake() != compared_normalized) ||
        compared.contains(['-', ' ']) || compared == "" || expected == ""
    {
        println!("{}  {}", compared, expected);
        return None;
    }

    let distance = edit_distance(&compared_normalized, &expected_normalized);
    let length = expected.len();
    let similarity = 100.0 * (1.0 - (distance as f64 /length as f64));

    if similarity > 50.0 {
        Some(format!("{:.0}%", similarity))
    } else {
        None
    }
}