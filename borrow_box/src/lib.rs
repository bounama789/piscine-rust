#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        })
    }
    pub fn read_winner(&self) -> (String, u16) {
        match self.p1.1.cmp(&self.p2.1) {
            std::cmp::Ordering::Less => self.p2.clone(),
            std::cmp::Ordering::Equal => ("Same score! tied".to_string(), self.p1.1),
            std::cmp::Ordering::Greater => self.p1.clone(),
        }
    }
    pub fn update_score(&mut self, user_name: String) {
        let x = self.nb_games/2;
        if self.p1.1 <= x && self.p2.1 <= x {
            if user_name == self.p1.0 {
                self.p1.1 += 1;
            } else if user_name == self.p2.0 {
                self.p2.1 += 1;
            }
        }
    }
    pub fn delete(self) -> String {
        format!("game deleted: id -> {}",self.id)
    }
}
