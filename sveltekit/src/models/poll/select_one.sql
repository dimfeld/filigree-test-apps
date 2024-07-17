SELECT
  id AS "id: PollId",
  organization_id AS "organization_id: crate::models::organization::OrganizationId",
  updated_at,
  created_at,
  question,
  answers,
  post_id AS "post_id: PostId"
FROM
  public.polls tb
WHERE
  id = $1
  AND tb.organization_id = $2
