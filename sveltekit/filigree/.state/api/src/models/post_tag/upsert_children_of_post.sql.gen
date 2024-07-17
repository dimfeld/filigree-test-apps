INSERT INTO public.post_tags (
  post_id,
  tag_id,
  organization_id)
VALUES
  __insertion_point_insert_values
ON CONFLICT (
  post_id)
  DO NOTHING
RETURNING
  post_id,
  tag_id,
  organization_id,
  updated_at,
  created_at
