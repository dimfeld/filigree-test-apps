UPDATE
  public.report_sections
SET
  name = $1,
  viz = $2,
  options = $3,
  report_id = $4,
  updated_at = NOW()
WHERE
  id = $5
  AND report_id = $6
  AND organization_id = $7
