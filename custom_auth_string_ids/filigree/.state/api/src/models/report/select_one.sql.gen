SELECT
  id AS "id: ReportId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at,
  title,
  description,
  ui
FROM
  myapp.reports tb
WHERE
  id = $1
  AND tb.organization_id = $2
