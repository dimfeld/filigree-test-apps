DELETE FROM public.post_images
WHERE organization_id = $1
  AND post_id = $2
