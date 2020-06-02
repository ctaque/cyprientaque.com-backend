-- Your SQL goes here

CREATE TABLE if not exists users (
  id serial primary key,
  name varchar(255) NOT NULL,
  slug varchar(255) NOT NULL,
  email varchar(255) NOT NULL,
  password varchar(255) NOT NULL,
  punchline varchar(50),
  website_url text,
  admin boolean DEFAULT false NOT NULL,
  active boolean DEFAULT true NOT NULL,
  deleted_at timestamp(0) without time zone,
  remember_token varchar(100),
  created_at timestamp(0) without time zone,
  updated_at timestamp(0) without time zone,
  api_token varchar(60)
);
