pub fn get_diamond(c: char) -> Vec<String> {
    let mut diamond = Vec::new();
    if c < 'A' || c > 'Z' {
        return diamond;
    }

    let size = (c as u8 - 'A' as u8) as usize;

    for i in 0..=size {
        let char = ('A' as u8 + i as u8) as char;
        let mut line = String::new();
        let spaces_outside = size - i;
        let spaces_inside = if i == 0 { 0 } else { 2 * i - 1 };

        line.push_str(&" ".repeat(spaces_outside));
        line.push(char);
        if spaces_inside > 0 {
            line.push_str(&" ".repeat(spaces_inside));
            line.push(char);
        }
        line.push_str(&" ".repeat(spaces_outside));

        diamond.push(line);
    }

    let mut bottom_half = diamond[..size].to_vec();
    bottom_half.reverse();
    diamond.extend(bottom_half);

    diamond
}
