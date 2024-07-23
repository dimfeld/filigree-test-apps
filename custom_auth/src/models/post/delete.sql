DELETE FROM myapp.posts
WHERE id = $1
  AND organization_id = $2
