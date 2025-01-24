use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};

pub mod create;
pub mod list;
pub mod promo_by_id;

#[derive(FromRow)]
pub struct Promo {
    pub description: String,
    pub image_url: Option<String>,
    pub target: Json<Target>,
    pub max_count: u32,
    pub active_from: Option<String>,
    pub active_until: Option<String>,
    pub mode: String,
    pub promo_common: Option<String>,
    pub promo_unique: Option<Json<Vec<String>>>,
    pub promo_id: String,
    pub company_id: String,
    pub company_name: String,
    pub likes: Json<HashSet<String>>,
    pub used_count: u32,
    pub active: bool,
    pub countries: Json<Vec<Country>>,
    pub comments: Json<HashSet<Comment>>,
    pub activated_users: Json<HashSet<String>>,
}

#[derive(Serialize, FromRow)]
pub struct PromoReadOnly {
    description: String,
    image_url: Option<String>,
    target: Json<Target>,
    max_count: u32,
    active_from: Option<String>,
    active_until: Option<String>,
    mode: String,
    promo_common: Option<String>,
    promo_unique: Option<Json<Vec<String>>>,
    promo_id: String,
    company_id: String,
    company_name: String,
    like_count: u32,
    used_count: u32,
    active: bool,
}

#[derive(Serialize)]
pub struct PromoForUser {
    pub promo_id: String,
    pub company_id: String,
    pub company_name: String,
    pub description: String,
    pub image_url: Option<String>,
    pub active: bool,
    pub is_activated_by_user: bool,
    pub like_count: u32,
    pub is_liked_by_user: bool,
    pub comment_count: u32,
}

#[derive(Deserialize)]
pub struct PatchPromo {
    description: Option<String>,
    image_url: Option<String>,
    target: Option<Json<Target>>,
    max_count: Option<u32>,
    active_from: Option<String>,
    active_until: Option<String>,
}

#[derive(Serialize)]
pub struct PromoStat {
    activate_count: u32,
    countries: Json<Vec<Country>>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreatePromo {
    description: Option<String>,
    image_url: Option<String>,
    target: Option<Target>,
    max_count: Option<u32>,
    active_from: Option<String>,
    active_until: Option<String>,
    mode: Option<String>,
    promo_common: Option<String>,
    promo_unique: Option<Json<Vec<String>>>,
}

impl CreatePromo {
    pub fn is_valid(&self) -> bool {
        if self.description.is_none()
            || self.mode.is_none()
            || self.max_count.is_none()
            || self.target.is_none()
        {
            return false;
        }

        if self.mode.as_ref().unwrap() != "COMMON" && self.mode.as_ref().unwrap() != "UNIQUE" {
            return false;
        } else if self.mode.as_ref().unwrap() == "COMMON"
            && (self.promo_common.as_ref().unwrap().len() < 5
                || self.promo_common.as_ref().unwrap().len() > 30)
        {
            return false;
        } else if self.mode.as_ref().unwrap() == "UNIQUE"
            && (self.promo_unique.as_ref().unwrap().is_empty()
                || self.promo_unique.as_ref().unwrap().len() > 5000)
        {
            return false;
        }
        if self.description.is_some() {
            if self.description.as_ref().unwrap().len() < 10
                || self.description.as_ref().unwrap().len() > 300
            {
                return false;
            }
        }
        true
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Target {
    age_from: Option<u8>,
    age_until: Option<u8>,
    country: Option<String>,
    categories: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Country {
    name: String,
    activate_count: u32,
}

#[derive(Clone, Serialize, Deserialize, FromRow, Hash, PartialEq, Eq)]
pub struct Comment {
    pub id: String,
    pub text: String,
    pub date: String,
    pub author: CommentAuthor,
}

#[derive(Clone, Serialize, Deserialize, FromRow, Hash, PartialEq, Eq)]
pub struct CommentAuthor {
    pub name: String,
    pub surname: String,
    pub avatar_url: Option<String>,
}
