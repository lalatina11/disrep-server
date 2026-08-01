-- Add migration script here
CREATE TABLE users(
    id VARCHAR(36) PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
)