CREATE TABLE to_do_items (
    id SERIAL PRIMARY KEY,
    title TEXT UNIQUE NOT NULL,
    status TEXT NOT NULL
);
-- Add migration script here
