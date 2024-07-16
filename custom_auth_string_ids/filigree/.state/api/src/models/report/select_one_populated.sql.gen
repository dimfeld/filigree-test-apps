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
      COALESCE(ARRAY_AGG(JSONB_BUILD_OBJECT('id', id, 'organization_id', organization_id,
	'updated_at', updated_at, 'created_at', created_at, 'name', name,
	'viz', viz, 'options', options, 'report_id', report_id)),
	ARRAY[]::jsonb[])
    FROM
      myapp.report_sections
    WHERE
      report_id = $1
      AND organization_id = $2) AS "report_sections!: Vec<ReportSection>"
FROM
  myapp.reports tb
WHERE
  tb.id = $1
  AND tb.organization_id = $2
