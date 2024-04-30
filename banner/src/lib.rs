use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        Flag {
            short_hand: format!("-{}",l_h.get(0..1).unwrap()),
            long_hand: l_h.to_string(),
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
        self.flags.insert((flag.0, flag.1), func);
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        let key =  self.flags.keys().find(|(sh,lh)| *sh == flag.0 || *lh == flag.1).unwrap();
        let func = self.flags.get(key).unwrap();
        
        func(argv[0], argv[1]).unwrap_or_else(|e| e.to_string())
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num1 = a.parse::<f32>()?;
    let num2 = b.parse::<f32>()?;
    Ok((num1 / num2).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num1: f32 = a.parse()?;
    let num2: f32 = b.parse()?;
    Ok((num1 % num2).to_string())
}
