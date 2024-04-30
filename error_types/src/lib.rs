pub use chrono::{NaiveDate, Utc};

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}
impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        let mut res = Vec::new();

        if self.first_name.is_empty() {
            return Err::<Vec<&str>, FormError>(FormError::new(
                "first_name".to_string(),
                self.first_name.clone(),
                "No user name".to_string(),
            ));
        }
        res.push("Valid first name");

        if self.password.len() < 8 {
            return Err::<Vec<&str>, FormError>(FormError::new(
                "password".to_string(),
                self.password.clone(),
                "At least 8 characters".to_string(),
            ));
        }
        if !self.password.chars().any(|c| c.is_numeric())
            || !self.password.chars().any(|c| c.is_alphabetic())
            || !self.password.chars().any(|c| !c.is_alphanumeric())
        {
            return Err::<Vec<&str>, FormError>(FormError::new(
                "password".to_string(),
                self.password.clone(),
                "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string(),
            ));
        };

        res.push("Valid password");
        return Ok(res);
    }
}
