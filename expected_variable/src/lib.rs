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

// fn edit_distance(s1: &str, s2: &str) -> usize {
//     let mut costs = (0..=s2.len()).collect::<Vec<_>>();

//     for (i, c1) in s1.chars().enumerate() {
//         let mut last_cost = i;
//         let mut new_costs = vec![i + 1];

//         for (j, c2) in s2.chars().enumerate() {
//             let replace_cost = if c1 == c2 { last_cost } else { last_cost + 1 };
//             let insert_cost = costs[j + 1] + 1;
//             let delete_cost = new_costs[j] + 1;

//             last_cost = replace_cost.min(insert_cost).min(delete_cost);
//             new_costs.push(last_cost);
//         }

//         costs = new_costs;
//     }

//     *costs.last().unwrap()
// }

fn is_camel_case(s: &str) -> bool {
    let mut prev_is_lower = true;
    let mut has_upper = false;

    let ch: Vec<char> = s.chars().collect();

    if !ch[0].is_uppercase() {
        return false;
    }

    for c in s.chars() {
        println!("{}", c);

        if c.is_uppercase() {
            if prev_is_lower {
                has_upper = true;
                prev_is_lower = false;
            } else {
                return false;
            }
            continue;
        }
        prev_is_lower = c.is_lowercase();
    }

    println!("{}", has_upper);

    true
}

fn is_snake_case(s: &str) -> bool {
    let mut prev_is_lower = true;
    let mut has_upper = false;

    let ch: Vec<char> = s.chars().collect();

    if !ch[0].is_uppercase() {
        return false;
    }

    for c in s.chars() {
        println!("{}", c);

        if c.is_uppercase() {
            if prev_is_lower {
                has_upper = true;
                prev_is_lower = false;
            } else {
                return false;
            }
            continue;
        }
        prev_is_lower = c.is_lowercase();
    }

    println!("{}", has_upper);

    true
}
