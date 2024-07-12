CREATE OR REPLACE FUNCTION objectid_to_uuid (text)
  RETURNS uuid
  LANGUAGE sql
  IMMUTABLE
  RETURNS NULL ON NULL INPUT PARALLEL SAFE
  AS $$
  SELECT
    encode(decode(replace(replace(
	  RIGHT ($1, 22), '-', '+'), '_', '/') ||
	    '==', 'base64'), 'hex')::uuid
$$;

CREATE OR REPLACE FUNCTION uuid_to_objectid (uuid)
  RETURNS text
  LANGUAGE sql
  IMMUTABLE
  RETURNS NULL ON NULL INPUT PARALLEL SAFE
  AS $$
  SELECT
    replace(replace(
      LEFT (encode(decode(replace($1::text, '-', ''), 'hex'),
	'base64'), 22), '+', '-'), '/', '_')
$$;

CREATE TABLE delete_log (
  organization_id uuid NOT NULL,
  object_id uuid NOT NULL,
  object_type text NOT NULL,
  data jsonb NOT NULL,
  deleted_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE organizations (
  id uuid NOT NULL PRIMARY KEY,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  name text NOT NULL,
  owner UUID REFERENCES users (id) ON DELETE SET NULL DEFERRABLE INITIALLY IMMEDIATE,
  default_role uuid,
  active boolean NOT NULL DEFAULT TRUE
);

CREATE TABLE users (
  id uuid NOT NULL PRIMARY KEY,
  organization_id uuid,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  name text NOT NULL,
  password_hash text,
  email text UNIQUE,
  avatar_url text
);

CREATE INDEX users_organization_id ON users (organization_id);

CREATE TABLE roles (
  id uuid NOT NULL PRIMARY KEY,
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  name text NOT NULL,
  description text
);

CREATE INDEX roles_organization_id ON roles (organization_id);

CREATE TABLE reports (
  id uuid NOT NULL PRIMARY KEY,
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  title text NOT NULL,
  description text,
  ui jsonb NOT NULL
);

CREATE INDEX reports_organization_id ON reports (organization_id);

CREATE INDEX reports_updated_at ON reports (organization_id, updated_at DESC);

CREATE INDEX reports_created_at ON reports (organization_id, created_at DESC);

CREATE TABLE report_sections (
  id uuid NOT NULL PRIMARY KEY,
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  name text NOT NULL,
  viz text NOT NULL,
  options jsonb NOT NULL,
  report_id uuid NOT NULL REFERENCES reports (id) ON DELETE CASCADE
);

CREATE INDEX report_sections_organization_id ON report_sections (organization_id);

CREATE INDEX report_sections_report_id ON report_sections (organization_id, report_id);

CREATE INDEX report_sections_updated_at ON report_sections (organization_id, updated_at DESC);

CREATE INDEX report_sections_created_at ON report_sections (organization_id, created_at DESC);

CREATE TABLE posts (
  id uuid NOT NULL PRIMARY KEY,
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  subject text NOT NULL,
  body text NOT NULL
);

CREATE INDEX posts_organization_id ON posts (organization_id);

CREATE INDEX posts_updated_at ON posts (organization_id, updated_at DESC);

CREATE INDEX posts_created_at ON posts (organization_id, created_at DESC);

CREATE TABLE reactions (
  id uuid NOT NULL PRIMARY KEY,
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  type TEXT NOT NULL,
  post_id uuid NOT NULL REFERENCES posts (id) ON DELETE CASCADE
);

CREATE INDEX reactions_organization_id ON reactions (organization_id);

CREATE INDEX reactions_post_id ON reactions (organization_id, post_id);

CREATE INDEX reactions_updated_at ON reactions (organization_id, updated_at DESC);

CREATE INDEX reactions_created_at ON reactions (organization_id, created_at DESC);

CREATE TABLE post_images (
  id uuid NOT NULL PRIMARY KEY,
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  file_storage_key text NOT NULL,
  file_storage_bucket text NOT NULL,
  file_original_name text,
  file_size bigint,
  file_hash bytea,
  post_id uuid NOT NULL REFERENCES posts (id) ON DELETE CASCADE
);

CREATE INDEX post_images_organization_id ON post_images (organization_id);

CREATE UNIQUE INDEX post_images_post_id ON post_images (organization_id, post_id);

CREATE INDEX post_images_updated_at ON post_images (organization_id, updated_at DESC);

CREATE INDEX post_images_created_at ON post_images (organization_id, created_at DESC);

CREATE TABLE polls (
  id uuid NOT NULL PRIMARY KEY,
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  question text NOT NULL,
  answers jsonb NOT NULL,
  post_id uuid NOT NULL REFERENCES posts (id) ON DELETE CASCADE
);

CREATE INDEX polls_organization_id ON polls (organization_id);

CREATE UNIQUE INDEX polls_post_id ON polls (organization_id, post_id);

CREATE INDEX polls_updated_at ON polls (organization_id, updated_at DESC);

CREATE INDEX polls_created_at ON polls (organization_id, created_at DESC);

CREATE TABLE comments (
  id uuid NOT NULL PRIMARY KEY,
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  body text NOT NULL,
  post_id uuid NOT NULL REFERENCES posts (id) ON DELETE CASCADE
);

CREATE INDEX comments_organization_id ON comments (organization_id);

CREATE INDEX comments_post_id ON comments (organization_id, post_id);

CREATE INDEX comments_updated_at ON comments (organization_id, updated_at DESC);

CREATE INDEX comments_created_at ON comments (organization_id, created_at DESC);

CREATE TABLE user_roles (
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE DEFERRABLE INITIALLY IMMEDIATE,
  user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE DEFERRABLE INITIALLY IMMEDIATE,
  role_id uuid NOT NULL REFERENCES roles (id) ON DELETE CASCADE DEFERRABLE INITIALLY IMMEDIATE,
  PRIMARY KEY (organization_id, user_id, role_id)
);

CREATE TABLE user_sessions (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
  hash UUID NOT NULL,
  expires_at timestamptz NOT NULL
);

CREATE TABLE organization_members (
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE DEFERRABLE INITIALLY IMMEDIATE,
  user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE DEFERRABLE INITIALLY IMMEDIATE,
  active boolean NOT NULL DEFAULT TRUE,
  PRIMARY KEY (organization_id, user_id)
);

CREATE INDEX user_sessions_user_id ON user_sessions (user_id);

CREATE TABLE api_keys (
  api_key_id uuid PRIMARY KEY,
  hash BYTEA NOT NULL,
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
  user_id uuid REFERENCES users (id) ON DELETE CASCADE,
  inherits_user_permissions bool NOT NULL DEFAULT FALSE,
  description text NOT NULL DEFAULT '',
  active boolean NOT NULL DEFAULT TRUE,
  expires_at timestamptz NOT NULL
);

CREATE TABLE email_logins (
  email text PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE DEFERRABLE INITIALLY IMMEDIATE,
  verified bool NOT NULL,
  reset_token uuid,
  reset_expires_at timestamptz,
  passwordless_login_token uuid,
  passwordless_login_expires_at timestamptz
);

CREATE INDEX email_logins_user_id ON email_logins (user_id);

CREATE TABLE oauth_logins (
  oauth_provider text NOT NULL,
  oauth_account_id text NOT NULL,
  user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE DEFERRABLE INITIALLY IMMEDIATE,
  PRIMARY KEY (oauth_provider, oauth_account_id)
);

CREATE INDEX oauth_logins_user_id ON oauth_logins (user_id);

CREATE TABLE oauth_authorization_sessions (
  key TEXT PRIMARY KEY,
  provider text NOT NULL,
  pkce_verifier text,
  add_to_user_id uuid,
  redirect_to text,
  expires_at timestamptz NOT NULL
);

CREATE TABLE user_invites (
  email text NOT NULL,
  token uuid NOT NULL,
  token_expires_at timestamptz NOT NULL,
  name text,
  invited_by uuid,
  organization_id uuid,
  role_ids uuid[],
  invite_sent_at timestamptz NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX user_invites_email_org ON user_invites (email, organization_id) NULLS NOT DISTINCT;

CREATE TABLE permissions (
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
  actor_id uuid NOT NULL,
  permission text NOT NULL,
  PRIMARY KEY (organization_id, actor_id, permission)
);

CREATE TABLE object_permissions (
  organization_id uuid NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
  actor_id uuid NOT NULL,
  object_id uuid NOT NULL,
  permission text NOT NULL,
  PRIMARY KEY (organization_id, actor_id, object_id, permission)
);
