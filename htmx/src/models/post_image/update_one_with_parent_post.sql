UPDATE
  public.post_images
SET
  file_storage_key = $1,
  file_storage_bucket = $2,
  file_original_name = $3,
  file_size = $4,
  file_hash = $5,
  post_id = $6,
  updated_at = NOW()
WHERE
  id = $7
  AND post_id = $8
  AND organization_id = $9
