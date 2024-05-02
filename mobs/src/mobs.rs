pub mod boss;
pub mod member;

use boss::Boss;
use member::{Member, Role};

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        self.members.push(Member::new(name, Role::Associate, age));
    }

    pub fn attack(&mut self, other: &mut Mob) {
        let self_power = self
            .members
            .iter()
            .map(|m| m.role.clone() as u32)
            .sum::<u32>();
        let other_power = other
            .members
            .iter()
            .map(|m| m.role.clone() as u32)
            .sum::<u32>();
        let mut winner: Option<&mut Mob> = None;
        let mut loser: Option<&mut Mob> = None;
        if self_power == other_power {
            self.members.pop();
        } else {
            if winner.is_none() || loser.is_none() {
                if self_power < other_power {
                    winner = Some(other);
                    loser = Some(self);
                } else {
                    winner = Some(other);
                    loser = Some(self);
                };
            }
        }

        if let Some(ref mut l) = loser {
            l.members.pop();

            if l.members.is_empty() {
                if let Some(w) = winner {
                    w.cities.append(&mut l.cities);
                    w.wealth += l.wealth;
                }
            }
        }
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u32) {
        let actual_amount = amount.min(target.wealth);
        target.wealth -= actual_amount;
        self.wealth += actual_amount;
    }

    pub fn conquer_city(&mut self, mobs: Vec<Mob>, city_name: String, value: u8) {
        if !mobs
            .iter()
            .any(|m| m.cities.iter().any(|(name, _)| name == &city_name))
        {
            self.cities.push((city_name, value));
        }
    }
}
