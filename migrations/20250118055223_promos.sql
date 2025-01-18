CREATE TABLE promos (
    id INTEGER NOT NULL,
    description TEXT,
    target JSON NOT NULL,
    max_count INTEGER NOT NULL,
    active_from TEXT,
    active_until TEXT,
    mode TEXT NOT NULL,
    promo_common TEXT,
    promo_unique JSON
);
