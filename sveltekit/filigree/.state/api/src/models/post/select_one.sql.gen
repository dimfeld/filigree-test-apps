SELECT
  id AS "id: PostId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at,
  subject,
  body
FROM
  public.posts tb
WHERE
  tb.id = $1
  AND tb.organization_id = $2
