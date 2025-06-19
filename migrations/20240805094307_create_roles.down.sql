-- Add down migration script here
ALTER TABLE users DROP CONSTRAINT IF EXISTS fk_users_roles;
ALTER TABLE users DROP COLUMN role_id;
DROP TABLE IF EXISTS "roles";
