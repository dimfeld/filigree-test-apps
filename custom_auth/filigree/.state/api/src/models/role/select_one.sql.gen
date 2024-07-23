SELECT
  id AS "id: RoleId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at,
  name,
  description,
  external_auth_provider,
  external_auth_id
FROM
  myapp.roles tb
WHERE
  id = $1
  AND tb.organization_id = $2
