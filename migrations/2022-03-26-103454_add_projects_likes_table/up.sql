-- Your SQL goes here
--
CREATE TABLE IF NOT EXISTS public.projects_likes (
   id SERIAL PRIMARY KEY,
   project_id integer NOT NULL REFERENCES projects (id),
   created_at timestamp(0) without time zone,
   ip inet not null
);
