INSERT INTO public.tags (
  id,
  organization_id,
  name,
  color)
VALUES (
  $1,
  $2,
  $3,
  $4)
RETURNING
  id AS "id: TagId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at,
  name,
  color
