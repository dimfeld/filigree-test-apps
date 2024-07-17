UPDATE
  myapp.comments
SET
  body = $1,
  post_id = $2,
  updated_at = NOW()
WHERE
  id = $3
  AND post_id = $4
  AND organization_id = $5
