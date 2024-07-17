SELECT
  id AS "id: ReportSectionId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at,
  name,
  viz,
  options,
  report_id AS "report_id: ReportId"
FROM
  myapp.report_sections tb
WHERE
  id = $1
  AND tb.organization_id = $2
