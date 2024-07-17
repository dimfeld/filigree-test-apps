DELETE FROM public.post_tags
WHERE organization_id = $1
  AND post_id = $2
  AND tag_id <> ALL ($3)
