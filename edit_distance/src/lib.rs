pub fn edit_distance(source: &str, target: &str) -> usize {
    let len_source = source.chars().count();
    let len_target = target.chars().count();

    let mut board = vec![vec![0; len_target+1]; len_source+1];

    for i in 0..len_source {
        board[i][0] = i;
    }

    for i in 0..len_target {
        board[0][i] = i;
    }

    for (i, c1) in source.chars().enumerate() {
        for (j, c2) in target.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };

            board[i + 1][j + 1] =
                *vec![board[i][j + 1] + 1, board[i + 1][j] + 1, board[i][j] + cost]
                    .iter()
                    .min()
                    .unwrap();
        }
    }

    board[len_source][len_target]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_distance() {
        let source = "alignment";
	let target = "assignment";
        let result = edit_distance(source, target);
        assert_eq!(result, 2);
    }
}
