DELETE FROM public.report_sections
WHERE organization_id = $1
  AND report_id = $2
  AND id <> ALL ($3)
