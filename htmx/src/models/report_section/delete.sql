DELETE FROM public.report_sections
WHERE id = $1
  AND organization_id = $2
