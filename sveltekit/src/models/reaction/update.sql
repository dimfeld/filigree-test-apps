UPDATE
  public.reactions
SET
  type = $2,
  post_id = $3,
  updated_at = NOW()
WHERE
  id = $1
  AND organization_id = $4
