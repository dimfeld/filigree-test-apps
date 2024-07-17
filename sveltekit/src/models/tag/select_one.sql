SELECT
  id AS "id: TagId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at,
  name,
  color
FROM
  public.tags tb
WHERE
  id = $1
  AND tb.organization_id = $2
