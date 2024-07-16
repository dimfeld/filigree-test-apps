SELECT
  id,
  organization_id,
  updated_at,
  created_at,
  body,
  post_id
FROM
  public.comments tb
WHERE
  organization_id = $1
  AND __insertion_point_filters
LIMIT $2 OFFSET $3
