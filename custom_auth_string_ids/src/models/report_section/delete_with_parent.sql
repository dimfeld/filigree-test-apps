DELETE FROM myapp.report_sections
WHERE organization_id = $1
  AND report_id = $2
  AND id = $3
