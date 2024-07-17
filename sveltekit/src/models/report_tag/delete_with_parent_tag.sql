DELETE FROM public.report_tags
WHERE organization_id = $1
  AND tag_id = $2
  AND report_id = $3
  AND tag_id = $4
