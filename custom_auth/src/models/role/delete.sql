DELETE FROM myapp.roles
WHERE id = $1
  AND organization_id = $2
