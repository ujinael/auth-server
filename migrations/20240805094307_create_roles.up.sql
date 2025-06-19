-- Add up migration script here
CREATE table roles(
id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
permissions JSONB NOT NULL,
title VARCHAR NOT NULL,
description VARCHAR NOT NULL,
created_at TIMESTAMP NOT NULL DEFAULT now(),
updated_at TIMESTAMP NOT NULL DEFAULT now(),
deleted_at TIMESTAMP DEFAULT NULL
);
ALTER TABLE users ADD COLUMN role_id UUID;
ALTER TABLE users ADD CONSTRAINT fk_users_roles FOREIGN KEY (role_id) REFERENCES roles (id);
