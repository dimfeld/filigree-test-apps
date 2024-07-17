SELECT
  id AS "id: ReportId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at,
  title,
  description,
  ui,
  (
    SELECT
      COALESCE(ARRAY_AGG(JSONB_BUILD_OBJECT('id', t.id, 'organization_id',
	t.organization_id, 'updated_at', t.updated_at, 'created_at', t.created_at,
	'name', t.name, 'viz', t.viz, 'options', t.options, 'report_id',
	t.report_id)), ARRAY[]::jsonb[])
    FROM
      public.report_sections t
    WHERE
      report_id = $1
      AND t.organization_id = $2) AS "report_sections!: Vec<ReportSection>"
FROM
  public.reports tb
WHERE
  id = $1
  AND tb.organization_id = $2
