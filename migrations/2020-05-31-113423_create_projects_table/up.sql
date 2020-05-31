-- Your SQL goes here
create table if not exists projects (
  id serial primary key,
  category_id integer not null references project_categories (id),
  title varchar(100) not null,
  slug varchar(255) not null,
  content text not null,
  views_count integer DEFAULT 0 NOT NULL,
  likes_count integer DEFAULT 0 NOT NULL,
  deleted_at timestamp(0) without time zone,
  created_at timestamp(0) without time zone,
  updated_at timestamp(0) without time zone
);
