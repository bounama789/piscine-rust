use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,

}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        Flag {
            short_hand: l_h.to_string(),
            long_hand: d.to_string(),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {

        self.flags.insert(flag, func);
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        

    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num1 = a.parse::<f32>().map_err(|_| ParseFloatError::new())?;
    let num2 = b.parse::<f32>().map_err(|_| ParseFloatError::new())?;
    if num2 == 0.0 {
        Err(ParseFloatError::new())
    } else {
        Ok((num1 / num2).to_string())
    }

}
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {

}