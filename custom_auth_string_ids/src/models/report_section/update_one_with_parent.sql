UPDATE
  myapp.report_sections
SET
  name = $2,
  viz = $3,
  options = $4,
  report_id = $5,
  updated_at = NOW()
WHERE
  id = $1
  AND report_id = $6
  AND organization_id = $7
