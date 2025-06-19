-- Add up migration script here
ALTER TABLE users ADD COLUMN is_active BOOL DEFAULT TRUE;
