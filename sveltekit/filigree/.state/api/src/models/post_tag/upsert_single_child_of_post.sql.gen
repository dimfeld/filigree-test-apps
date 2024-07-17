INSERT INTO public.post_tags (
  post_id,
  tag_id,
  organization_id)
VALUES (
  $1,
  $2,
  $3)
ON CONFLICT (
  post_id)
  DO UPDATE SET
    tag_id = EXCLUDED.tag_id,
    updated_at = now()
  WHERE
    post_tags.organization_id = $3
    AND post_tags.post_id = $4
  RETURNING
    post_id AS "post_id: PostId",
    tag_id AS "tag_id: TagId",
    organization_id AS "organization_id: crate::models::organization::OrganizationId",
    updated_at,
    created_at
