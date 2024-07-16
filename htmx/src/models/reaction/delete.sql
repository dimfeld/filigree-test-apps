DELETE FROM public.reactions
WHERE id = $1
  AND organization_id = $2
