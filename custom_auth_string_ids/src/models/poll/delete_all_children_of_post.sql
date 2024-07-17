DELETE FROM myapp.polls
WHERE organization_id = $1
  AND post_id = $2
