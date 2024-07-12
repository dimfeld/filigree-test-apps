WITH permissions AS (
  SELECT
    COALESCE(bool_or(permission IN ('org_admin', 'Post::owner')), FALSE) AS is_owner,
    COALESCE(bool_or(permission IN ('org_admin', 'Post::owner', 'Post::write')), FALSE) AS is_user
  FROM
    myapp.permissions
  WHERE
    organization_id = $2
    AND actor_id = ANY ($3)
    AND permission IN ('org_admin', 'Post::owner', 'Post::write'))
UPDATE
  myapp.posts
SET
  subject = $4,
  body = $5,
  updated_at = now()
FROM
  permissions
WHERE
  id = $1
  AND organization_id = $2
  AND (permissions.is_owner
    OR permissions.is_user)
RETURNING
  permissions.is_owner AS "is_owner!"
