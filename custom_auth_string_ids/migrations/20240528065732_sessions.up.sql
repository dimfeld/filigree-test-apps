CREATE TABLE myapp.user_sessions (
  id text PRIMARY KEY,
  user_id text NOT NULL,
  expires_at timestamptz NOT NULL
);
