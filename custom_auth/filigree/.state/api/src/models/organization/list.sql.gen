SELECT
  id,
  updated_at,
  created_at,
  name,
  OWNER,
  default_role,
  active,
  external_auth_provider,
  external_auth_id
FROM
  myapp.organizations tb
WHERE
  __insertion_point_filters
LIMIT $2 OFFSET $3
