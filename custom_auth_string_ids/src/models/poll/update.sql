UPDATE
  myapp.polls
SET
  question = $2,
  answers = $3,
  post_id = $4,
  updated_at = NOW()
WHERE
  id = $1
  AND organization_id = $5
