INSERT INTO public.report_tags (
  report_id,
  tag_id,
  organization_id)
VALUES
  __insertion_point_insert_values
ON CONFLICT (
  report_id,
  tag_id)
  DO NOTHING
RETURNING
  report_id,
  tag_id,
  organization_id,
  updated_at,
  created_at
