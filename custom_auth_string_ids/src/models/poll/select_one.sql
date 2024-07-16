SELECT
  id AS "id: PollId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at,
  question,
  answers,
  post_id AS "post_id: PostId"
FROM
  myapp.polls tb
WHERE
  tb.id = $1
  AND tb.organization_id = $2
