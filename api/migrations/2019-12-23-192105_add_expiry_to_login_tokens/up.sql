ALTER TABLE login_tokens
  ADD COLUMN expiry TIMESTAMP NOT NULL DEFAULT NOW() + INTERVAL '1 minute';