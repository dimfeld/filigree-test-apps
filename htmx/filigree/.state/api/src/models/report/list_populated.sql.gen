SELECT
  id,
  organization_id,
  updated_at,
  created_at,
  title,
  description,
  ui,
  (
    SELECT
      COALESCE(ARRAY_AGG(report_sections.id), ARRAY[]::uuid[])
    FROM
      public.report_sections
    WHERE
      report_id = tb.id
      AND organization_id = $1) AS "report_section_ids"
FROM
  public.reports tb
WHERE
  organization_id = $1
  AND __insertion_point_filters
LIMIT $2 OFFSET $3
