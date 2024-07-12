DELETE FROM myapp.organizations
WHERE id = $1
  AND EXISTS (
    SELECT
      1
    FROM
      myapp.permissions
    WHERE
      organization_id = $2
      AND actor_id = ANY ($3)
      AND permission IN ('org_admin', 'Organization::owner'))
