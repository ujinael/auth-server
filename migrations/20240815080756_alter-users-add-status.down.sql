-- Add down migration script here
ALTER TABLE users DROP COLUMN IF EXISTS is_active;
