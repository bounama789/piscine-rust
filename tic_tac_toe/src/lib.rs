pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    let players = ["X", "O"];
    for player in players.iter() {
        if diagonals(player, &table) || horizontal(player, &table) || vertical(player, &table) {
            return format!("player {} won", player);
        }
    }
    "tie".to_string()
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let size = table.len();
    let mut diag1 = true;
    let mut diag2 = true;
    for i in 0..size {
        if table[i][i] != player {
            diag1 = false;
        }
        if table[i][size - 1 - i] != player {
            diag2 = false;
        }
    }
    diag1 || diag2
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    table.iter().any(|row| row.iter().all(|&cell| cell == player))
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let size = table.len();
    (0..size).any(|col| (0..size).all(|row| table[row][col] == player))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tic_tac_toe() {
        let result = tic_tac_toe(vec![
            vec!["O", "X", "O"],
            vec!["O", "O", "X"],
            vec!["X", "#", "X"]
        ]);
        assert_eq!(result, "tie");

        let result = tic_tac_toe(vec![
            vec!["X", "O", "O"],
            vec!["X", "O", "O"],
            vec!["#", "O", "X"]
        ]);
        assert_eq!(result, "player O won");

        let result = tic_tac_toe(vec![
            vec!["O", "O", "X"],
            vec!["O", "X", "O"],
            vec!["X", "#", "X"]
        ]);
        assert_eq!(result,"player X won");
    }
}
