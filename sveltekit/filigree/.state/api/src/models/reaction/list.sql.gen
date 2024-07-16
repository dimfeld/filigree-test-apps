SELECT
  id,
  organization_id,
  updated_at,
  created_at,
  type,
  post_id
FROM
  public.reactions tb
WHERE
  organization_id = $1
  AND __insertion_point_filters
LIMIT $2 OFFSET $3
