SELECT
  report_id AS "report_id: ReportId",
  tag_id AS "tag_id: TagId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at
FROM
  public.report_tags tb
WHERE
  report_id = $2
  AND tag_id = $3
  AND tb.organization_id = $1
