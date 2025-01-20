CREATE TABLE users (
    name TEXT NOT NULL,
    surname TEXT NOT NULL,
    email TEXT NOT NULL,
    avatar_url TEXT,
    other JSON,
    password_hash TEXT NOT NULL
);
