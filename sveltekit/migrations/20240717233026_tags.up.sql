CREATE TABLE public.tags (
  id uuid NOT NULL PRIMARY KEY,
  organization_id uuid NOT NULL REFERENCES public.organizations (id) ON DELETE CASCADE,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  name text NOT NULL,
  color text NOT NULL
);

CREATE INDEX tags_organization_id ON public.tags (organization_id);

CREATE INDEX tags_updated_at ON public.tags (organization_id, updated_at DESC);

CREATE INDEX tags_created_at ON public.tags (organization_id, created_at DESC);

CREATE TABLE public.report_tags (
  report_id uuid NOT NULL REFERENCES public.reports (id) ON DELETE CASCADE DEFERRABLE INITIALLY IMMEDIATE,
  tag_id uuid NOT NULL REFERENCES public.tags (id) ON DELETE CASCADE DEFERRABLE INITIALLY IMMEDIATE,
  organization_id uuid NOT NULL REFERENCES public.organizations (id) ON DELETE CASCADE,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (report_id, tag_id)
);

CREATE INDEX report_tags_report_id ON public.report_tags (organization_id, report_id);

CREATE INDEX report_tags_tag_id ON public.report_tags (organization_id, tag_id);

CREATE INDEX report_tags_organization_id ON public.report_tags (organization_id);

CREATE INDEX report_tags_updated_at ON public.report_tags (organization_id, updated_at DESC);

CREATE INDEX report_tags_created_at ON public.report_tags (organization_id, created_at DESC);

CREATE TABLE public.post_tags (
  post_id uuid NOT NULL REFERENCES public.posts (id) ON DELETE CASCADE DEFERRABLE INITIALLY IMMEDIATE,
  tag_id uuid NOT NULL REFERENCES public.tags (id) ON DELETE CASCADE DEFERRABLE INITIALLY IMMEDIATE,
  organization_id uuid NOT NULL REFERENCES public.organizations (id) ON DELETE CASCADE,
  updated_at timestamptz NOT NULL DEFAULT now(),
  created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (post_id, tag_id)
);

CREATE INDEX post_tags_post_id ON public.post_tags (organization_id, post_id);

CREATE INDEX post_tags_tag_id ON public.post_tags (organization_id, tag_id);

CREATE INDEX post_tags_organization_id ON public.post_tags (organization_id);

CREATE INDEX post_tags_updated_at ON public.post_tags (organization_id, updated_at DESC);

CREATE INDEX post_tags_created_at ON public.post_tags (organization_id, created_at DESC);
