INSERT INTO public.post_tags (
  post_id,
  tag_id,
  organization_id)
VALUES (
  $1,
  $2,
  $3)
ON CONFLICT (
  post_id,
  tag_id)
  DO NOTHING
RETURNING
  post_id AS "post_id: PostId",
  tag_id AS "tag_id: TagId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at
