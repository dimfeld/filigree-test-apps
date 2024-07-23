UPDATE
  myapp.posts
SET
  subject = $1,
  body = $2,
  updated_at = NOW()
WHERE
  id = $3
  AND organization_id = $4
