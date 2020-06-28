-- Your SQL goes here
alter table project_images add column if not exists original_object_url varchar(255);
