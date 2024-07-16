UPDATE
  public.post_images
SET
  file_storage_key = $2,
  file_storage_bucket = $3,
  file_original_name = $4,
  file_size = $5,
  file_hash = $6,
  post_id = $7,
  updated_at = NOW()
WHERE
  id = $1
  AND organization_id = $8
