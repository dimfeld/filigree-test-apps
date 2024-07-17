INSERT INTO public.report_tags (
  report_id,
  tag_id,
  organization_id)
VALUES (
  $1,
  $2,
  $3)
RETURNING
  report_id AS "report_id: ReportId",
  tag_id AS "tag_id: TagId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at
