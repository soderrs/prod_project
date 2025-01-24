CREATE TABLE promos (
    description TEXT NOT NULL,
    image_url TEXT,
    target JSON NOT NULL,
    max_count INTEGER NOT NULL,
    active_from TEXT,
    active_until TEXT,
    mode TEXT NOT NULL,
    promo_common TEXT,
    promo_unique JSON,
    promo_id TEXT NOT NULL,
    company_id TEXT NOT NULL,
    company_name TEXT NOT NULL,
    likes JSON,
    used_count INTEGER ,
    active BOOLEAN ,
    countries JSON,
    comments JSON,
    activated_users JSON 
);

