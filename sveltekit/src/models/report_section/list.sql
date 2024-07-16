SELECT
  id,
  organization_id,
  updated_at,
  created_at,
  name,
  viz,
  options,
  report_id
FROM
  public.report_sections tb
WHERE
  organization_id = $1
  AND __insertion_point_filters
LIMIT $2 OFFSET $3
