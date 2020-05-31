-- Your SQL goes here
CREATE TABLE if not exists project_images (
  id serial primary key,
  w1500_keyname character varying(255) NOT NULL,
  w350_keyname character varying(255) NOT NULL,
  w1500_object_url character varying(255) NOT NULL,
  w350_object_url character varying(255) NOT NULL,
  "primary" boolean DEFAULT false NOT NULL,
  project_id integer NOT NULL references projects(id),
  created_at timestamp(0) without time zone,
  updated_at timestamp(0) without time zone
);
