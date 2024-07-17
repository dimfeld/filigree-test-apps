INSERT INTO public.reactions (
  id,
  organization_id,
  type,
  post_id)
VALUES
  __insertion_point_insert_values
ON CONFLICT (
  id)
  DO UPDATE SET type = EXCLUDED.type, post_id = EXCLUDED.post_id, updated_at = now()
  WHERE
    reactions.organization_id = $1 AND reactions.post_id = $2
  RETURNING
    id, organization_id, updated_at, created_at, type, post_id
