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
      public.comments ct
    WHERE
      ct.post_id = tb.id
      AND organization_id = $1) AS "comment_ids",
  (
    SELECT
      ct.id
    FROM
      public.polls ct
    WHERE
      ct.post_id = tb.id
      AND organization_id = $1
    LIMIT 1) AS "poll_id",
(
  SELECT
    ct.tag_id
  FROM
    public.post_tags ct
  WHERE
    ct.post_id = tb.id
    AND organization_id = $1
  LIMIT 1) AS "tag_id"
FROM
  public.posts tb
WHERE
  organization_id = $1
  AND __insertion_point_filters
LIMIT $2 OFFSET $3
