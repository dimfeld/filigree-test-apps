UPDATE
  public.reactions
SET
  type = $1,
  post_id = $2,
  updated_at = NOW()
WHERE
  id = $3
  AND organization_id = $4
