DELETE FROM public.post_images
WHERE id = $1
  AND organization_id = $2
