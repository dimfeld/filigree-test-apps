DROP INDEX myapp.roles_external_auth_id;

DROP INDEX myapp.organizations_external_auth_id;

DROP INDEX myapp.users_external_auth_id;

ALTER TABLE IF EXISTS myapp.polls
  ALTER COLUMN organization_id SET DATA TYPE TEXT;

ALTER TABLE myapp.users
  DROP COLUMN external_auth_id CASCADE;

ALTER TABLE myapp.users
  DROP COLUMN external_auth_provider CASCADE;

ALTER TABLE IF EXISTS myapp.users
  ALTER COLUMN organization_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.users
  ALTER COLUMN id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.reports
  ALTER COLUMN organization_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.delete_log
  ALTER COLUMN organization_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.report_sections
  ALTER COLUMN organization_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.post_images
  ALTER COLUMN organization_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.comments
  ALTER COLUMN organization_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.permissions
  ALTER COLUMN actor_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.permissions
  ALTER COLUMN organization_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.reactions
  ALTER COLUMN organization_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.posts
  ALTER COLUMN organization_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.user_roles
  ALTER COLUMN role_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.user_roles
  ALTER COLUMN user_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.user_roles
  ALTER COLUMN organization_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.user_sessions
  ALTER COLUMN user_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.user_sessions
  ALTER COLUMN id SET DATA TYPE TEXT;

ALTER TABLE myapp.organizations
  DROP COLUMN external_auth_id CASCADE;

ALTER TABLE myapp.organizations
  DROP COLUMN external_auth_provider CASCADE;

ALTER TABLE IF EXISTS myapp.organizations
  ALTER COLUMN default_role SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.organizations
  ALTER COLUMN OWNER SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.organizations
  ALTER COLUMN id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.object_permissions
  ALTER COLUMN actor_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.object_permissions
  ALTER COLUMN organization_id SET DATA TYPE TEXT;

ALTER TABLE myapp.roles
  DROP COLUMN external_auth_id CASCADE;

ALTER TABLE myapp.roles
  DROP COLUMN external_auth_provider CASCADE;

ALTER TABLE IF EXISTS myapp.roles
  ALTER COLUMN organization_id SET DATA TYPE TEXT;

ALTER TABLE IF EXISTS myapp.roles
  ALTER COLUMN id SET DATA TYPE TEXT;
