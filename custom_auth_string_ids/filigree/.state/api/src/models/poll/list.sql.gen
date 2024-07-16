SELECT
  id,
  organization_id,
  updated_at,
  created_at,
  question,
  answers,
  post_id
FROM
  myapp.polls tb
WHERE
  organization_id = $1
  AND __insertion_point_filters
LIMIT $2 OFFSET $3
