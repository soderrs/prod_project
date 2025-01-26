use std::{collections::HashSet, str::FromStr};

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};

pub mod create;
pub mod list;
pub mod promo_by_id;

#[derive(FromRow, Clone)]
pub struct Promo {
    pub description: String,
    pub image_url: Option<String>,
    pub target: Json<Target>,
    pub max_count: i32,
    pub create_date: Json<DateTime<Utc>>,
    pub active_from: Option<Json<NaiveDate>>,
    pub active_until: Option<Json<NaiveDate>>,
    pub mode: String,
    pub promo_common: Option<String>,
    pub promo_unique: Option<Json<Vec<String>>>,
    pub promo_id: String,
    pub company_id: String,
    pub company_name: String,
    pub likes: Json<HashSet<String>>,
    pub used_count: i32,
    pub active: bool,
    pub countries: Json<Vec<Country>>,
    pub comments: Json<HashSet<Comment>>,
    pub activated_users: Json<HashSet<String>>,
}

#[derive(Serialize, FromRow)]
pub struct PromoReadOnly {
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,
    target: Json<Target>,
    max_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_from: Option<Json<NaiveDate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    active_until: Option<Json<NaiveDate>>,
    mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    promo_common: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    promo_unique: Option<Json<Vec<String>>>,
    promo_id: String,
    company_id: String,
    company_name: String,
    like_count: i32,
    used_count: i32,
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
    pub like_count: i32,
    pub is_liked_by_user: bool,
    pub comment_count: i32,
}

#[derive(Deserialize)]
pub struct PatchPromo {
    description: Option<String>,
    image_url: Option<String>,
    target: Option<Json<Target>>,
    max_count: Option<i32>,
    active_from: Option<Json<String>>,
    active_until: Option<Json<String>>,
}

impl PatchPromo {
    pub fn is_valid(&self, promo: &Promo) -> bool {
        if let Some(ref description) = self.description {
            if description.len() < 10 || description.len() > 300 {
                return false;
            }
        }
        if let Some(ref image_url) = self.image_url {
            if image_url.len() > 350 {
                return false;
            }
        }
        if let Some(max_count) = self.max_count {
            if max_count as i32 > promo.max_count {
                return false;
            } else if promo.mode == "UNIQUE" && max_count != 1 {
                return false;
            }
        }
        if let Some(ref target) = self.target {
            return target.is_valid();
        }

        if let Some(ref active_from) = self.active_from {
            match NaiveDate::from_str(&active_from) {
                Ok(_) => (),
                Err(_) => return false,
            };
        }

        if let Some(ref active_until) = self.active_until {
            match NaiveDate::from_str(&active_until) {
                Ok(_) => (),
                Err(_) => return false,
            };
        }

        if self.active_from.is_some() && self.active_until.is_some() {
            if NaiveDate::from_str(self.active_from.as_ref().unwrap()).unwrap()
                > NaiveDate::from_str(self.active_until.as_ref().unwrap()).unwrap()
            {
                return false;
            }
        }
        true
    }
}

#[derive(Serialize)]
pub struct PromoStat {
    activate_count: i32,
    countries: Json<Vec<Country>>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreatePromo {
    description: Option<String>,
    image_url: Option<String>,
    target: Option<Target>,
    max_count: Option<i32>,
    active_from: Option<Json<NaiveDate>>,
    active_until: Option<Json<NaiveDate>>,
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
        } else if (self.mode.as_ref().unwrap() == "COMMON" && self.promo_common.is_none())
            || (self.mode.as_ref().unwrap() == "UNIQUE" && self.promo_unique.is_none())
        {
            return false;
        } else if self.mode.as_ref().unwrap() == "COMMON"
            && (self.promo_common.as_ref().unwrap().chars().count() < 5
                || self.promo_common.as_ref().unwrap().chars().count() > 30)
        {
            return false;
        } else if self.mode.as_ref().unwrap() == "UNIQUE"
            && (self.promo_unique.as_ref().unwrap().is_empty()
                || self.promo_unique.as_ref().unwrap().len() > 5000)
        {
            return false;
        } else if self.description.as_ref().unwrap().chars().count() < 10
            || self.description.as_ref().unwrap().chars().count() > 300
        {
            return false;
        } else if self.promo_unique.is_some() {
            return self
                .promo_unique
                .as_ref()
                .unwrap()
                .0
                .iter()
                .filter(|promo| promo.chars().count() < 3 || promo.chars().count() > 30)
                .collect::<Vec<&String>>()
                .is_empty();
        }
        true
    }
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Target {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_from: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_until: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
}

impl Target {
    pub fn is_valid(&self) -> bool {
        if let Some(age_from) = self.age_from {
            if age_from < 0 || age_from > 100 {
                return false;
            }
        }
        if let Some(age_until) = self.age_until {
            if age_until < 0 || age_until > 100 {
                return false;
            }
        }
        if self.age_from.is_some() && self.age_until.is_some() {
            if self.age_from.unwrap() > self.age_until.unwrap() {
                return false;
            }
        }
        if let Some(ref country) = self.country {
            if country.chars().count() != 2 {
                return false;
            }
        }
        if let Some(ref categories) = self.categories {
            if categories.len() > 20
                || !categories
                    .iter()
                    .filter(|category| category.len() < 2 || category.len() > 20)
                    .collect::<Vec<&String>>()
                    .is_empty()
            {
                return false;
            }
        }

        true
    }
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Country {
    pub name: String,
    pub activate_count: i32,
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
    pub email: String,
    pub avatar_url: Option<String>,
}
