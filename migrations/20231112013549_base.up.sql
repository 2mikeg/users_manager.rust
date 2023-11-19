-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
        id uuid default uuid_generate_v1() PRIMARY KEY NOT NULL,
        username VARCHAR(255) NOT NULL UNIQUE,
        hashed_password VARCHAR(255) NOT NULL,
        created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
    )