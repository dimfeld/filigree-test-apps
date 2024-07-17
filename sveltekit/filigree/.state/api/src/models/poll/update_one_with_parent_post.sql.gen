UPDATE
  public.polls
SET
  question = $1,
  answers = $2,
  post_id = $3,
  updated_at = NOW()
WHERE
  id = $4
  AND post_id = $5
  AND organization_id = $6
