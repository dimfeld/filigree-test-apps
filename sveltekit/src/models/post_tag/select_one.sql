SELECT
  post_id AS "post_id: PostId",
  tag_id AS "tag_id: TagId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at
FROM
  public.post_tags tb
WHERE
  post_id = $2
  AND tag_id = $3
  AND tb.organization_id = $1
