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

        if self_power < other_power || self_power == other_power {
            self.members.pop();
        } else {
            other.members.pop();
        }

        if other.members.is_empty() {
            self.wealth += other.wealth;
            other.wealth = 0;
            self.cities.append(&mut other.cities);
        } else if self.members.is_empty() {
            other.wealth += self.wealth;
            self.wealth = 0;
            other.cities.append(&mut self.cities);
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
