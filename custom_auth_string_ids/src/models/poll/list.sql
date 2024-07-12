SELECT
  id,
  organization_id,
  updated_at,
  created_at,
  question,
  answers,
  post_id,
  perm._permission
FROM
  myapp.polls tb
  JOIN LATERAL (
    SELECT
      CASE WHEN bool_or(permission IN ('org_admin', 'Poll::owner')) THEN
        'owner'
      WHEN bool_or(permission = 'Poll::write') THEN
        'write'
      WHEN bool_or(permission = 'Poll::read') THEN
        'read'
      ELSE
        NULL
      END _permission
    FROM
      myapp.permissions
    WHERE
      organization_id = $1
      AND actor_id = ANY ($2)
      AND permission IN ('org_admin', 'Poll::owner', 'Poll::write', 'Poll::read')) perm ON
	perm._permission IS NOT NULL
WHERE
  organization_id = $1
  AND __insertion_point_filters
ORDER BY
  __insertion_point_order_by
LIMIT $3 OFFSET $4