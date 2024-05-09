#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}



#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;
impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

impl Display for Antigen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{:?}",self))
    }
}

impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(RhFactor::Negative),
            "+" => Ok(RhFactor::Positive),
            _ => Err(()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.antigen.cmp(&other.antigen) == Ordering::Equal {
            self.rh_factor.cmp(&other.rh_factor)
        } else {
            self.antigen.cmp(&other.antigen)
        }
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ant: Antigen = s.get(..s.len() - 1).unwrap().parse()?;
        let fac: RhFactor = s.get(s.len() - 1..).unwrap().parse()?;

        return Ok(BloodType {
            antigen: ant,
            rh_factor: fac,
        });
    }
}

use std::fmt::{self, Debug, Display};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fact = if self.rh_factor == RhFactor::Negative{'-'}else{'+'};
        f.write_str(&format!("{}{}",self.antigen,fact))
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        match (other.antigen.clone(), other.rh_factor.clone()) {
            (Antigen::A, RhFactor::Positive) => {
                (other.antigen == self.antigen
                    || self.antigen == Antigen::AB) && self.rh_factor == RhFactor::Positive
            }
            (Antigen::A, RhFactor::Negative) => {
                other.antigen == self.antigen || self.antigen == Antigen::AB
            }
            (Antigen::B, RhFactor::Positive) => {
                (other.antigen == self.antigen
                    || self.antigen == Antigen::AB) && self.rh_factor == RhFactor::Positive
            }
            (Antigen::B, RhFactor::Negative) => {
                other.antigen == self.antigen || self.antigen == Antigen::AB
            }
            (Antigen::O, RhFactor::Positive) => other.rh_factor == self.rh_factor,
            (Antigen::O, RhFactor::Negative) => true,
            (Antigen::AB, RhFactor::Positive) => {
                other.antigen == self.antigen && self.rh_factor == other.rh_factor
            }
            (Antigen::AB, RhFactor::Negative) => other.antigen == self.antigen,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut bloodtypes = get_all_bloodtypes();
        bloodtypes.retain(|b| self.can_receive_from(b));
        bloodtypes
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut bloodtypes = get_all_bloodtypes();
        bloodtypes.retain(|b| b.can_receive_from(self));
        bloodtypes
    }
}


fn get_all_bloodtypes()-> Vec<BloodType>{
    let antigenes: Vec<Antigen> = vec![Antigen::A,Antigen::AB,Antigen::B,Antigen::O];
    let mut res = Vec::new();
    antigenes.iter().for_each(|a|{
        res.push(BloodType{
            antigen:a.clone(),
            rh_factor:RhFactor::Positive
        });
        res.push(BloodType{
            antigen:a.clone(),
            rh_factor:RhFactor::Negative
        });
    });
    res
}
