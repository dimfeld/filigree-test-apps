UPDATE
  myapp.reactions
SET
  type = $2,
  post_id = $3,
  updated_at = NOW()
WHERE
  id = $1
  AND post_id = $4
  AND organization_id = $5
