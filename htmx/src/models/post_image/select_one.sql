SELECT
  id AS "id: PostImageId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at,
  file_storage_key,
  file_storage_bucket,
  file_original_name,
  file_size,
  file_hash,
  post_id AS "post_id: PostId"
FROM
  public.post_images tb
WHERE
  id = $1
  AND tb.organization_id = $2
