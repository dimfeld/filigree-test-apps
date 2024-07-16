SELECT
  id,
  organization_id,
  updated_at,
  created_at,
  subject,
  body,
  (
    SELECT
      COALESCE(ARRAY_AGG(comments.id), ARRAY[]::uuid[])
    FROM
      myapp.comments
    WHERE
      post_id = tb.id
      AND organization_id = $1) AS "comment_ids",
  (
    SELECT
      polls.id
    FROM
      myapp.polls
    WHERE
      post_id = tb.id
      AND organization_id = $1
    LIMIT 1) AS "poll_id"
FROM
  myapp.posts tb
WHERE
  organization_id = $1
  AND __insertion_point_filters
LIMIT $2 OFFSET $3
