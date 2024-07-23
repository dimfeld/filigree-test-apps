CREATE INDEX users_external_auth_id ON myapp.users (organization_id, external_auth_id);

CREATE INDEX organizations_external_auth_id ON myapp.organizations (external_auth_id);

CREATE INDEX roles_external_auth_id ON myapp.roles (organization_id, external_auth_id);

ALTER TABLE myapp.polls
  ALTER COLUMN organization_id SET DATA TYPE UUID;

ALTER TABLE myapp.users
  ALTER COLUMN id SET DATA TYPE UUID;

ALTER TABLE myapp.users
  ALTER COLUMN organization_id SET DATA TYPE UUID;

ALTER TABLE myapp.users
  ADD COLUMN external_auth_provider text;

ALTER TABLE myapp.users
  ADD COLUMN external_auth_id text;

ALTER TABLE myapp.reports
  ALTER COLUMN organization_id SET DATA TYPE UUID;

ALTER TABLE myapp.delete_log
  ALTER COLUMN organization_id SET DATA TYPE UUID;

ALTER TABLE myapp.report_sections
  ALTER COLUMN organization_id SET DATA TYPE UUID;

ALTER TABLE myapp.post_images
  ALTER COLUMN organization_id SET DATA TYPE UUID;

ALTER TABLE myapp.comments
  ALTER COLUMN organization_id SET DATA TYPE UUID;

ALTER TABLE myapp.permissions
  ALTER COLUMN organization_id SET DATA TYPE UUID;

ALTER TABLE myapp.permissions
  ALTER COLUMN actor_id SET DATA TYPE UUID;

ALTER TABLE myapp.reactions
  ALTER COLUMN organization_id SET DATA TYPE UUID;

ALTER TABLE myapp.posts
  ALTER COLUMN organization_id SET DATA TYPE UUID;

ALTER TABLE myapp.user_roles
  ALTER COLUMN organization_id SET DATA TYPE UUID;

ALTER TABLE myapp.user_roles
  ALTER COLUMN user_id SET DATA TYPE UUID;

ALTER TABLE myapp.user_roles
  ALTER COLUMN role_id SET DATA TYPE UUID;

ALTER TABLE myapp.user_sessions
  ALTER COLUMN id SET DATA TYPE UUID;

ALTER TABLE myapp.user_sessions
  ALTER COLUMN user_id SET DATA TYPE UUID;

ALTER TABLE myapp.organizations
  ALTER COLUMN id SET DATA TYPE UUID;

ALTER TABLE myapp.organizations
  ALTER COLUMN OWNER SET DATA TYPE UUID;

ALTER TABLE myapp.organizations
  ALTER COLUMN default_role SET DATA TYPE UUID;

ALTER TABLE myapp.organizations
  ADD COLUMN external_auth_provider text;

ALTER TABLE myapp.organizations
  ADD COLUMN external_auth_id text;

ALTER TABLE myapp.object_permissions
  ALTER COLUMN organization_id SET DATA TYPE UUID;

ALTER TABLE myapp.object_permissions
  ALTER COLUMN actor_id SET DATA TYPE UUID;

ALTER TABLE myapp.roles
  ALTER COLUMN id SET DATA TYPE UUID;

ALTER TABLE myapp.roles
  ALTER COLUMN organization_id SET DATA TYPE UUID;

ALTER TABLE myapp.roles
  ADD COLUMN external_auth_provider text;

ALTER TABLE myapp.roles
  ADD COLUMN external_auth_id text;
