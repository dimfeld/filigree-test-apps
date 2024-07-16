UPDATE
  myapp.organizations
SET
  name = $2,
  OWNER = $3,
  default_role = $4,
  updated_at = NOW()
WHERE
  id = $1
