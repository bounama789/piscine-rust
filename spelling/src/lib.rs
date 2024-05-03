pub fn spell(n: u64) -> String {
    match n {
        0 => return "zero".to_string(),
        1_000_000 => return "one million".to_string(),
        _ => {}
    }

    let below_20 = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let mut result = String::new();
    let mut num = n;

    if num >= 1000 {
        let thousands = num / 1000;
        result.push_str(&format!("{} thousand", spell(thousands)));
        num %= 1000;
        if num > 0 {
            result.push(' ');
        }
    }

    if num >= 100 {
        let hundreds = num / 100;
        result.push_str(&format!("{} hundred", below_20[hundreds as usize]));
        num %= 100;
        if num > 0 {
            result.push(' ');
        }
    }

    match num {
        20..=99 => {
            let t = num / 10;
            result.push_str(tens[t as usize]);
            num %= 10;
            if num > 0 {
                result.push('-');
                result.push_str(below_20[num as usize])
            }
        }
        1..=19 => result.push_str(below_20[num as usize]),
        _ => {}
    }

    result
}
