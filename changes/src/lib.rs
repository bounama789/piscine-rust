#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        Light {
            alias: alias.to_string(),
            brightness: 0,
        }
    }
}

pub fn change_brightness(lights: &mut Vec<Light>, alias: &str, value: u8) {
    lights.iter_mut().for_each(|l| {
        if l.alias.eq(alias) {
            l.brightness = value;
        }
    });
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_brightness() {
        // bedroom
        let mut lights = vec![
            Light::new("living_room"),
            Light::new("bedroom"),
            Light::new("rest_room"),
        ];
        assert_eq!(lights[0].brightness, 0);

        change_brightness(&mut lights, "living_room", 200);
        assert_eq!(lights[0].brightness, 200);
    }
}
