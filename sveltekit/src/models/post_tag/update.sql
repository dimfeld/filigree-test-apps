UPDATE
  public.post_tags
SET
  updated_at = NOW()
WHERE
  post_id = $1
  AND tag_id = $2
  AND organization_id = $3
