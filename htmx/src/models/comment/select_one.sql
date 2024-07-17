SELECT
  id AS "id: CommentId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at,
  body,
  post_id AS "post_id: PostId"
FROM
  public.comments tb
WHERE
  id = $1
  AND tb.organization_id = $2
