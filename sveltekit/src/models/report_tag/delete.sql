DELETE FROM public.report_tags
WHERE report_id = $1
  AND tag_id = $2
  AND organization_id = $3
