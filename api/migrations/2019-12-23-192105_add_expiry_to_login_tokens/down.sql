-- This file should undo anything in `up.sql`
ALTER TABLE login_tokens DROP COLUMN expiry;