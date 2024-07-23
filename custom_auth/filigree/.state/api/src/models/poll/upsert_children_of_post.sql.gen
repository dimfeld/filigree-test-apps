INSERT INTO myapp.polls (
  id,
  organization_id,
  question,
  answers,
  post_id)
VALUES
  __insertion_point_insert_values
ON CONFLICT (
  post_id)
  DO UPDATE SET
    question = EXCLUDED.question,
    answers = EXCLUDED.answers,
    post_id = EXCLUDED.post_id,
    updated_at = now()
  WHERE
    polls.organization_id = $1
    AND polls.post_id = $2
  RETURNING
    id,
    organization_id,
    updated_at,
    created_at,
    question,
    answers,
    post_id
