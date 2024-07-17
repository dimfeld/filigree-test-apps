SELECT
  post_id,
  tag_id,
  organization_id,
  updated_at,
  created_at
FROM
  public.post_tags tb
WHERE
  organization_id = $1
  AND __insertion_point_filters
LIMIT $2 OFFSET $3
