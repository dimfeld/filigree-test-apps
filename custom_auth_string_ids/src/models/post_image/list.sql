SELECT
  id,
  organization_id,
  updated_at,
  created_at,
  file_storage_key,
  file_storage_bucket,
  file_original_name,
  file_size,
  file_hash,
  post_id
FROM
  myapp.post_images tb
WHERE
  organization_id = $1
  AND __insertion_point_filters
LIMIT $2 OFFSET $3
