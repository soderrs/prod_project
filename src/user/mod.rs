use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod auth;
pub mod feed;
pub mod middlewares;
pub mod profile;
pub mod promo;

#[derive(Serialize, Deserialize, FromRow)]
pub struct UserProfile {
    name: String,
    surname: String,
    email: String,
    avatar_url: Option<String>,
    other: sqlx::types::Json<UserTargetSettings>,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    name: String,
    surname: String,
    email: String,
    avatar_url: Option<String>,
    other: sqlx::types::Json<UserTargetSettings>,
    password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct PatchUser {
    name: Option<String>,
    surname: Option<String>,
    avatar_url: Option<String>,
    password: Option<String>,
}

impl PatchUser {
    pub fn is_valid(&self) -> bool {
        if let Some(ref name) = self.name {
            if name.len() < 1 || name.len() > 100 {
                return false;
            }
        }
        if let Some(ref surname) = self.surname {
            if surname.len() < 1 || surname.len() > 120 {
                return false;
            }
        }

        if self.avatar_url.is_some() && self.avatar_url.as_ref().unwrap().len() > 350 {
            return false;
        }
        if let Some(ref password) = self.password {
            let mut has_whitespace = false;
            let mut has_upper = false;
            let mut has_lower = false;
            let mut has_digit = false;
            for c in password.chars() {
                has_whitespace |= c.is_whitespace();
                has_lower |= c.is_lowercase();
                has_upper |= c.is_uppercase();
                has_digit |= c.is_digit(10);
            }

            return !has_whitespace
                && has_upper
                && has_lower
                && has_digit
                && password.len() >= 8
                && password.len() <= 60;
        }
        true
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateUser {
    name: String,
    surname: String,
    email: String,
    avatar_url: Option<String>,
    other: sqlx::types::Json<UserTargetSettings>,
    password: String,
}

impl CreateUser {
    pub fn is_valid(&self) -> bool {
        if self.name.len() < 1 || self.name.len() > 100 {
            return false;
        }
        if self.surname.len() < 1 || self.surname.len() > 120 {
            return false;
        }
        let email_regex = Regex::new(r"[a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?").unwrap();
        if !email_regex.is_match(&self.email) {
            return false;
        }
        if self.avatar_url.is_some() && self.avatar_url.as_ref().unwrap().len() > 350 {
            return false;
        }
        if !self.other.is_valid() {
            return false;
        }
        let mut has_whitespace = false;
        let mut has_upper = false;
        let mut has_lower = false;
        let mut has_digit = false;

        for c in self.password.chars() {
            has_whitespace |= c.is_whitespace();
            has_lower |= c.is_lowercase();
            has_upper |= c.is_uppercase();
            has_digit |= c.is_digit(10);
        }

        !has_whitespace
            && has_upper
            && has_lower
            && has_digit
            && self.password.len() >= 8
            && self.password.len() <= 60
    }
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
struct UserTargetSettings {
    age: i32,
    country: String,
}

impl UserTargetSettings {
    pub fn is_valid(&self) -> bool {
        if self.age < 0 || self.age > 100 {
            return false;
        }
        if self.country.len() != 2 {
            return false;
        }
        true
    }
}
