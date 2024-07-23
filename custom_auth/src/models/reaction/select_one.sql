SELECT
  id AS "id: ReactionId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at,
  type AS "typ",
  post_id AS "post_id: PostId"
FROM
  myapp.reactions tb
WHERE
  id = $1
  AND tb.organization_id = $2
