SELECT
  id,
  organization_id,
  updated_at,
  created_at,
  name,
  description,
  external_auth_provider,
  external_auth_id
FROM
  myapp.roles tb
WHERE
  organization_id = $1
  AND __insertion_point_filters
LIMIT $2 OFFSET $3
