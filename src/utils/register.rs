use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub birthdate: i64,
    pub is_male: Option<bool>,
}

pub fn check_username(username: &str) -> String {
    match username.len() {
        5..=12 => (),
        _ => return "Le pseudo doit contenir entre 5 et 12 caractères compris.".to_string(),
    }
    for (i, c) in username.char_indices() {
        if i == 0 {
            if !c.is_alphabetic() {
                return "Le pseudo doit commencer par une lettre.".to_string();
            } else {
                if !c.is_alphanumeric() && c != '_' {
                    return "Le pseudo ne peut contenir que des lettres, des nombres et des underscores.".to_string();
                }
            }
        }
    }
    String::new()
}
