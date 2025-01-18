use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};

pub mod create;
pub mod list;
pub mod promo_by_id;

pub struct Promo {
    description: String,
    image_url: Option<String>,
    target: Target,
    max_count: u32,
    active_from: Option<String>,
    active_until: Option<String>,
    mode: String,
    promo_common: Option<String>,
    promo_unique: Option<Vec<String>>,
    promo_id: String,
    company_id: String,
    company_name: String,
    like_count: u32,
    used_count: u32,
    active: bool,
}

pub struct PromoReadOnly {
    description: String,
    image_url: Option<String>,
    target: Target,
    max_count: u32,
    active_from: Option<String>,
    active_until: Option<String>,
    mode: String,
    promo_common: Option<String>,
    promo_unique: Option<Vec<String>>,
    promo_id: String,
    company_id: String,
    company_name: String,
    like_count: u32,
    used_count: u32,
    active: bool,
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

#[derive(Serialize, Deserialize)]
struct Target {
    age_from: Option<u8>,
    age_until: Option<u8>,
    country: Option<String>,
    categories: Option<Vec<String>>,
}
