CREATE SCHEMA IF NOT EXISTS myapp;

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

CREATE TABLE IF NOT EXISTS myapp.delete_log (
  organization_id text NOT NULL,
  object_id uuid NOT NULL,
  object_type text NOT NULL,
  data jsonb NOT NULL,
  deleted_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE myapp.users (
  id text NOT NULL PRIMARY KEY,
  organization_id text,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  name text NOT NULL,
  password_hash text,
  email text UNIQUE,
  avatar_url text
);

CREATE INDEX users_organization_id ON myapp.users (organization_id);

CREATE TABLE myapp.organizations (
  id text NOT NULL PRIMARY KEY,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  name text NOT NULL,
  owner TEXT,
  default_role text,
  active boolean NOT NULL DEFAULT TRUE
);

CREATE TABLE myapp.roles (
  id text NOT NULL PRIMARY KEY,
  organization_id text NOT NULL,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  name text NOT NULL,
  description text
);

CREATE INDEX roles_organization_id ON myapp.roles (organization_id);

CREATE TABLE myapp.reports (
  id uuid NOT NULL PRIMARY KEY,
  organization_id text NOT NULL,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  title text NOT NULL,
  description text,
  ui jsonb NOT NULL
);

CREATE INDEX reports_organization_id ON myapp.reports (organization_id);

CREATE INDEX reports_updated_at ON myapp.reports (organization_id, updated_at DESC);

CREATE INDEX reports_created_at ON myapp.reports (organization_id, created_at DESC);

CREATE TABLE myapp.report_sections (
  id uuid NOT NULL PRIMARY KEY,
  organization_id text NOT NULL,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  name text NOT NULL,
  viz text NOT NULL,
  options jsonb NOT NULL,
  report_id uuid NOT NULL REFERENCES myapp.reports (id) ON DELETE CASCADE
);

CREATE INDEX report_sections_organization_id ON myapp.report_sections (organization_id);

CREATE INDEX report_sections_report_id ON myapp.report_sections (organization_id, report_id);

CREATE INDEX report_sections_updated_at ON myapp.report_sections (organization_id, updated_at DESC);

CREATE INDEX report_sections_created_at ON myapp.report_sections (organization_id, created_at DESC);

CREATE TABLE myapp.posts (
  id uuid NOT NULL PRIMARY KEY,
  organization_id text NOT NULL,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  subject text NOT NULL,
  body text NOT NULL
);

CREATE INDEX posts_organization_id ON myapp.posts (organization_id);

CREATE INDEX posts_subject ON myapp.posts (organization_id, subject);

CREATE INDEX posts_updated_at ON myapp.posts (organization_id, updated_at DESC);

CREATE INDEX posts_created_at ON myapp.posts (organization_id, created_at DESC);

CREATE TABLE myapp.reactions (
  id uuid NOT NULL PRIMARY KEY,
  organization_id text NOT NULL,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  type TEXT NOT NULL,
  post_id uuid NOT NULL REFERENCES myapp.posts (id) ON DELETE CASCADE
);

CREATE INDEX reactions_organization_id ON myapp.reactions (organization_id);

CREATE INDEX reactions_post_id ON myapp.reactions (organization_id, post_id);

CREATE INDEX reactions_updated_at ON myapp.reactions (organization_id, updated_at DESC);

CREATE INDEX reactions_created_at ON myapp.reactions (organization_id, created_at DESC);

CREATE TABLE myapp.post_images (
  id uuid NOT NULL PRIMARY KEY,
  organization_id text NOT NULL,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  file_storage_key text NOT NULL,
  file_storage_bucket text NOT NULL,
  file_original_name text,
  file_size bigint,
  file_hash bytea,
  post_id uuid NOT NULL REFERENCES myapp.posts (id) ON DELETE CASCADE
);

CREATE INDEX post_images_organization_id ON myapp.post_images (organization_id);

CREATE INDEX post_images_post_id ON myapp.post_images (organization_id, post_id);

CREATE INDEX post_images_updated_at ON myapp.post_images (organization_id, updated_at DESC);

CREATE INDEX post_images_created_at ON myapp.post_images (organization_id, created_at DESC);

CREATE TABLE myapp.polls (
  id uuid NOT NULL PRIMARY KEY,
  organization_id text NOT NULL,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  question text NOT NULL,
  answers jsonb NOT NULL,
  post_id uuid NOT NULL UNIQUE REFERENCES myapp.posts (id) ON DELETE CASCADE
);

CREATE INDEX polls_organization_id ON myapp.polls (organization_id);

CREATE INDEX polls_post_id ON myapp.polls (organization_id, post_id);

CREATE INDEX polls_updated_at ON myapp.polls (organization_id, updated_at DESC);

CREATE INDEX polls_created_at ON myapp.polls (organization_id, created_at DESC);

CREATE TABLE myapp.comments (
  id uuid NOT NULL PRIMARY KEY,
  organization_id text NOT NULL,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  body text NOT NULL,
  post_id uuid NOT NULL REFERENCES myapp.posts (id) ON DELETE CASCADE
);

CREATE INDEX comments_organization_id ON myapp.comments (organization_id);

CREATE INDEX comments_post_id ON myapp.comments (organization_id, post_id);

CREATE INDEX comments_updated_at ON myapp.comments (organization_id, updated_at DESC);

CREATE INDEX comments_created_at ON myapp.comments (organization_id, created_at DESC);

CREATE TABLE myapp.user_roles (
  organization_id text NOT NULL,
  user_id text NOT NULL,
  role_id text NOT NULL,
  PRIMARY KEY (organization_id, user_id, role_id)
);

CREATE TABLE myapp.permissions (
  organization_id text NOT NULL,
  actor_id text NOT NULL,
  permission text NOT NULL,
  PRIMARY KEY (organization_id, actor_id, permission)
);

CREATE TABLE myapp.object_permissions (
  organization_id text NOT NULL,
  actor_id text NOT NULL,
  object_id uuid NOT NULL,
  permission text NOT NULL,
  PRIMARY KEY (organization_id, actor_id, object_id, permission)
);
