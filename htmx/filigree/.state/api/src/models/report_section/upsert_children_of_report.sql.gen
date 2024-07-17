INSERT INTO public.report_sections (
  id,
  organization_id,
  name,
  viz,
  options,
  report_id)
VALUES
  __insertion_point_insert_values
ON CONFLICT (
  id)
  DO UPDATE SET
    name = EXCLUDED.name,
    viz = EXCLUDED.viz,
    options = EXCLUDED.options,
    report_id = EXCLUDED.report_id,
    updated_at = now()
  WHERE
    report_sections.organization_id = $1
    AND report_sections.report_id = $2
  RETURNING
    id,
    organization_id,
    updated_at,
    created_at,
    name,
    viz,
    options,
    report_id
