DELETE FROM myapp.reports
WHERE id = $1
  AND organization_id = $2
