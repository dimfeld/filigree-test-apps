SELECT
  id,
  organization_id,
  updated_at,
  created_at,
  subject,
  body,
  (
    SELECT
      COALESCE(ARRAY_AGG(ct.id), ARRAY[]::uuid[])
    FROM
      myapp.comments ct
    WHERE
      ct.post_id = tb.id
      AND organization_id = $1) AS "comment_ids",
  (
    SELECT
      ct.id
    FROM
      myapp.polls ct
    WHERE
      ct.post_id = tb.id
      AND organization_id = $1
    LIMIT 1) AS "poll_id"
FROM
  myapp.posts tb
WHERE
  organization_id = $1
  AND __insertion_point_filters
LIMIT $2 OFFSET $3
