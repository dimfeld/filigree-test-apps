INSERT INTO myapp.post_images (
  id,
  organization_id,
  file_storage_key,
  file_storage_bucket,
  file_original_name,
  file_size,
  file_hash,
  post_id)
VALUES
  __insertion_point_insert_values
ON CONFLICT (
  id)
  DO UPDATE SET
    file_storage_key = EXCLUDED.file_storage_key,
    file_storage_bucket = EXCLUDED.file_storage_bucket,
    file_original_name = EXCLUDED.file_original_name,
    file_size = EXCLUDED.file_size,
    file_hash = EXCLUDED.file_hash,
    post_id = EXCLUDED.post_id,
    updated_at = now()
  WHERE
    post_images.organization_id = $1
    AND post_images.post_id = $2
  RETURNING
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
