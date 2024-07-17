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
      COALESCE(ARRAY_AGG(ct.id), ARRAY[]::uuid[])
    FROM
      public.report_sections ct
    WHERE
      ct.report_id = tb.id
      AND organization_id = $1) AS "report_section_ids",
  (
    SELECT
      COALESCE(ARRAY_AGG(ct.tag_id), ARRAY[]::uuid[])
    FROM
      public.report_tags ct
    WHERE
      ct.report_id = tb.id
      AND organization_id = $1) AS "tag_ids"
FROM
  public.reports tb
WHERE
  organization_id = $1
  AND __insertion_point_filters
LIMIT $2 OFFSET $3
