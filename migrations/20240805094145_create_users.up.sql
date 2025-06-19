-- Add up migration script here
CREATE TYPE user_type AS ENUM ('employee', 'client', 'admin');

CREATE TABLE IF NOT EXISTS "users"(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    login VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL,
    permissions JSONB NOT NULL,
    data jsonb NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at TIMESTAMP DEFAULT NULL
);
