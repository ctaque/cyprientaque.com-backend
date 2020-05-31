-- Your SQL goes here
create table if not exists project_categories (
  id serial primary key,
  name varchar(255) not null,
  picture_url varchar(255) default null,
  slug varchar(255) unique not null
 );
