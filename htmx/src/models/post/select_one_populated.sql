SELECT
  id AS "id: PostId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at,
  subject,
  body,
  (
    SELECT
      COALESCE(ARRAY_AGG(ct.id), ARRAY[]::uuid[])
    FROM
      public.comments ct
    WHERE
      ct.post_id = $1
      AND organization_id = $2) AS "comment_ids!: Vec<CommentId>",
  (
    SELECT
      COALESCE(ARRAY_AGG(JSONB_BUILD_OBJECT('id', t.id, 'organization_id',
	t.organization_id, 'updated_at', t.updated_at, 'created_at', t.created_at,
	'typ', t.type, 'post_id', t.post_id)), ARRAY[]::jsonb[])
    FROM
      public.reactions t
    WHERE
      post_id = $1
      AND t.organization_id = $2) AS "reactions!: Vec<Reaction>",
  (
    SELECT
      JSONB_BUILD_OBJECT('id', t.id, 'organization_id', t.organization_id, 'updated_at',
	t.updated_at, 'created_at', t.created_at, 'question', t.question, 'answers',
	t.answers, 'post_id', t.post_id)
    FROM
      public.polls t
    WHERE
      post_id = $1
      AND t.organization_id = $2
    LIMIT 1) AS "poll: Poll",
(
  SELECT
    COALESCE(ARRAY_AGG(JSONB_BUILD_OBJECT('id', t.id, 'organization_id',
      t.organization_id, 'updated_at', t.updated_at, 'created_at', t.created_at,
      'file_storage_key', t.file_storage_key, 'file_storage_bucket', t.file_storage_bucket, 'file_original_name',
      t.file_original_name, 'file_size', t.file_size, 'file_hash', t.file_hash,
      'post_id', t.post_id)), ARRAY[]::jsonb[])
  FROM
    public.post_images t
  WHERE
    post_id = $1
    AND t.organization_id = $2) AS "images!: Vec<PostImage>"
FROM
  public.posts tb
WHERE
  id = $1
  AND tb.organization_id = $2
