DELETE FROM public.report_tags
WHERE organization_id = $1
  AND report_id = $2
  AND tag_id <> ALL ($3)
